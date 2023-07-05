use std::time::{Duration, Instant};

pub struct Throttler {
    start_time: Instant,
    transaction_interval: Duration,
}

impl Throttler {
    pub fn new(tps: u32) -> Self {
        let transaction_interval = Duration::from_secs_f64(1.0 / tps as f64);
        Throttler {
            start_time: Instant::now(),
            transaction_interval,
        }
    }

    pub fn throttle(&self) {
        let elapsed_time = self.start_time.elapsed();

        if elapsed_time < self.transaction_interval {
            let remaining_time = self.transaction_interval - elapsed_time;
            std::thread::sleep(remaining_time);
        }
    }
}
