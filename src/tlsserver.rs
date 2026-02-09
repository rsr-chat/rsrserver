use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::PathBuf;

use std::sync::Arc;

use color_eyre::eyre::Result;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
use tokio_rustls::server::TlsStream;
use tokio_rustls::{TlsAcceptor, rustls};

pub struct TlsServerConfig {
    pub addr: SocketAddr,
    pub cert: PathBuf,
    pub key: PathBuf,
    pub pool_size: usize,
}

impl Default for TlsServerConfig {
    fn default() -> Self {
        Self {
            addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 443)),
            cert: PathBuf::new(),
            key: PathBuf::new(),
            pool_size: 128,
        }
    }
}

pub struct TlsServer {
    semaphore: Arc<Semaphore>,
    listener: TcpListener,
    acceptor: TlsAcceptor,
}

impl TlsServer {
    pub async fn create(cfg: TlsServerConfig) -> Result<Self> {
        let certs = CertificateDer::pem_file_iter(cfg.cert)?.collect::<Result<Vec<_>, _>>()?;
        let key = PrivateKeyDer::from_pem_file(cfg.key)?;

        // Build the TLS side of the server.
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;
        let acceptor = TlsAcceptor::from(Arc::new(config));

        // Create the server listener.
        let listener: TcpListener = TcpListener::bind(&cfg.addr).await?;

        // Semaphore to limit concurrent request processing
        let semaphore = Arc::new(Semaphore::new(cfg.pool_size));

        Ok(Self {
            semaphore,
            listener,
            acceptor,
        })
    }

    pub async fn serve<C, H>(&mut self, cfg: &'static C, handler: H)
    where
        C: AsyncFriendly,
        H: TlsHandler<C>,
    {
        // Main server loop to process incoming requests.
        loop {
            if let Err(e) = self.serve_once::<C, H>(&cfg, handler.clone()).await {
                println!("Error: {e}"); // TODO: Enhance error reporting
            }
        }
    }

    async fn serve_once<C, H>(&mut self, cfg: &'static C, handler: H) -> Result<()>
    where
        C: AsyncFriendly,
        H: TlsHandler<C>,
    {
        let (stream, client_addr) = self.listener.accept().await?;
        let acceptor = self.acceptor.clone();

        let accepted_stream = acceptor.accept(stream).await?;

        let semaphore = self.semaphore.clone();
        match semaphore.clone().try_acquire_owned() {
            Err(_) => {
                // No permits available - server at capacity
                send503(
                    accepted_stream,
                    "The server is over capacity. Please try again later.",
                )
                .await?;
            }
            Ok(permit) => {
                // Successfully acquired permit, spawn task
                tokio::spawn(async move {
                    let _permit = permit; // held until task completes
                    handler.handle(cfg, accepted_stream, client_addr).await;
                });
            }
        }
        Ok(())
    }
}

pub trait TlsHandler<C: AsyncFriendly>: Clone + Send + Sync + 'static {
    type Future: AsyncFuture<()>;

    fn handle(
        &self,
        cfg: &'static C,
        stream: TlsStream<TcpStream>,
        client_addr: SocketAddr,
    ) -> Self::Future;
}

// Blanket impl for any Fn closure
impl<C, F, Fut> TlsHandler<C> for F
where
    C: AsyncFriendly,
    Fut: AsyncFuture<()>,
    F: Fn(&'static C, TlsStream<TcpStream>, SocketAddr) -> Fut + Clone + Send + Sync + 'static,
{
    type Future = Fut;

    fn handle(
        &self,
        cfg: &'static C,
        stream: TlsStream<TcpStream>,
        client_addr: SocketAddr,
    ) -> Self::Future {
        self(cfg, stream, client_addr)
    }
}

pub trait AsyncFriendly: Send + Sync + 'static {}
impl<T> AsyncFriendly for T where T: Send + Sync + 'static {}

pub trait AsyncFuture<T>: Future<Output = T> + AsyncFriendly {}
impl<T, O> AsyncFuture<O> for T where T: Future<Output = O> + Send + Sync + 'static {}

async fn send503(mut stream: TlsStream<TcpStream>, message: &str) -> Result<()> {
    let response = format!(
        "HTTP/1.0 503 Service Unavailable\r\n\
        Connection: close\r\n\
        Content-length: {}\r\n\
        \r\n\
        {}",
        message.len(),
        message
    );
    stream.write_all(response.as_bytes()).await?;
    stream.shutdown().await?;
    Ok(())
}
