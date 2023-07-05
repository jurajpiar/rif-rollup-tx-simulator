use clap::{Command, Parser, arg};
use rand::prelude::*;

use crate::{config::Config, transaction::Transaction, throttler::Throttler};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub verbose: bool,
    pub config_file: Option<String>,
}

fn create_cli() -> Command {
    let app = Command::new("Simulation Tool")
        .version("0.1")
        .about("A CLI simulation tool for RIF Rollup");
    let verbose_arg = arg!(-v --verbose "Turns on more verbose logging");
    let config_arg = arg!(-c --config <FILE> "Overrides default configuration file");
    app.arg(verbose_arg);
    app.arg(config_arg);

    app
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            verbose: false,
            config_file: None
        }
    }

    pub fn run(&self) {
        let arguments = create_cli().get_matches();
        
        let config_file = arguments.get_one::<String>("config").unwrap_or(&String::from("config.toml"));
        let config = match Config::load_from_file(config_file) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error loading configuration: {}", err);
                return;
            }
        };

        // Start the simulation based on the configuration
        self.start_simulation(&config, &Client::new());
    }

    fn start_simulation(&self, config: &Config, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        // Generate transactions
        let mut rng = rand::thread_rng();
        let num_transactions = rng.gen_range(1..= config.general.tps);
    
        let throttler = Throttler::new(config.general.tps);

        for _ in 0..num_transactions {
            // Submit transaction
            
            // Throttle between transactions
            if config.general.enable_throttling {
                throttler.throttle();
            }
        }
    
        Ok(())
    }
    
    
}
