use log::{debug, error, log_enabled, info, Level};
use std::env;

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
    info!("Starting");

    let entries = gtf::read_gtk("data/Homo_sapiens.GRCh38.107.gtf").unwrap();
    for i in 0..10 {
        println!("{:?}", entries[i]);
    }

    info!("Finished");
}
