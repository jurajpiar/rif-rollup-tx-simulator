use rand::Rng;

use crate::config::{Config, TransactionConfig};

pub struct Transaction {
}

impl Transaction {
    pub fn generate_deposit(config: &Config) -> Self {
        let TransactionConfig {
            min_deposit_value, max_deposit_value, ..
        } = config.transaction;
        let mut rng = rand::thread_rng();
        let amount = rng.gen_range(config.transaction.min_deposit_value..= config.transaction.max_deposit_value);
        
        Transaction {
        }
    }
}
