

use crate::cli::Cli;

pub mod cli;
pub mod config;
pub mod transaction;
pub mod throttler;
pub mod rollup;

fn main() {
    let cli = Cli::new();

    cli.run();
}
