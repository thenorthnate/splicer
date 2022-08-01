use log::{error, info};
use std::env;

mod elements;
mod gtf;
mod trie;

fn main() {
    match env::var("RUST_LOG") {
        Ok(_) => {},
        Err(_) => {
            // Set the default log level to "info"
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
    info!("Starting processing");

    let entries = match gtf::read_gtf("data/Homo_sapiens.GRCh38.107.gtf") {
        Ok(v) => v,
        Err(e) => {
            error!("failed to read gtf file due to {}", e);
            return;
        }
    };
    for i in 0..10 {
        println!("{:?}", entries[i]);
    }

    info!("Finished");
}
