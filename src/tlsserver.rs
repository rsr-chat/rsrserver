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

    pub async fn serve<H>(&mut self, mut handler: H)
    where
        H: TlsHandler,
    {
        // Main server loop to process incoming requests.
        loop {
            if let Err(e) = self.serve_once::<H>(&mut handler).await {
                println!("Error: {e}"); // TODO: Enhance error reporting
            }
        }
    }

    async fn serve_once<H>(&mut self, handler: &mut H) -> Result<()>
    where
        H: TlsHandler,
    {
        let (stream, client_addr) = self.listener.accept().await?;
        let acceptor = self.acceptor.clone();

        let accepted_stream = acceptor.accept(stream).await?;

        tokio::spawn(handler.handle(accepted_stream, client_addr));

        Ok(())
    }
}

pub trait TlsHandler: Send + Sync + 'static {
    type Future: AsyncFuture<()>;

    fn handle(
        &mut self,
        stream: TlsStream<TcpStream>,
        client_addr: SocketAddr,
    ) -> Self::Future;
}

// Blanket impl for any Fn closure
impl<F, Fut> TlsHandler for F
where
    Fut: AsyncFuture<()>,
    F: Fn(TlsStream<TcpStream>, SocketAddr) -> Fut + Send + Sync + 'static,
{
    type Future = Fut;

    fn handle(
        &mut self,
        stream: TlsStream<TcpStream>,
        client_addr: SocketAddr,
    ) -> Self::Future {
        self(stream, client_addr)
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
