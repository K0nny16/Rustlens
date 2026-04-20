use clap::Parser;
use rustlens::cli::{Cli, run};

fn main() {
    let args = Cli::parse();
    if let Err(e) = run(args.command) {
        eprintln!("Something went wrong! \n {}", e);
    }
}
