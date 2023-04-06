use clap::Parser;
use minigrep::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = minigrep::run(config) {
        // write to stderr
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
