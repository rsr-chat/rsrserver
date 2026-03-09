use flatc_rust;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/ipc/packet.fbs");
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("src/ipc/packet.fbs")],
        out_dir: Path::new("src/ipc"),
        ..Default::default()
    }).expect("flatc");
}