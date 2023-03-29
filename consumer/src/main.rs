use tokio::time::{self, Duration};
use rand::Rng;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use crate::utils::server::Server;


mod utils;

const QUEUE_NAME: &'static str = "my_queue";
const OUTPUT_FILE: &'static str = "output.txt";
const RABBIT_STRING: &'static str = "amqp://localhost:5672/DSrabbitmq";

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    power_value: i32,
    timestamp: i64,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
    .with_target(false)
    .with_timer(tracing_subscriber::fmt::time::uptime())
    .with_level(true)
    .init();

    println!("Service C running...");

    loop {

        let server = Server::new(RABBIT_STRING, QUEUE_NAME, OUTPUT_FILE).await;
        // consumer.
        server.consume().await;
        time::sleep(Duration::from_secs(5)).await;
    }
}


// Testing the random number generator
#[cfg(test)]
mod tests {
    use super::*;

    fn generate_random_pv_value() -> u32 {
        // Get the current time
        let now = Local::now();
        // Calculate a multiplier based on the hour of the day
        let hour_multiplier = (now.hour() as f32 - 12.0) / 12.0;
        // Calculate the range of the random number based on the multiplier
        let range_size = (hour_multiplier * 45.0).abs() as u32;
        let max_value = 1200;
        let min_value = if range_size > max_value { 0 } else { max_value - range_size };
        // Generate a random value within the range
        println!("min -> {}, max -> {}", min_value, max_value);
        let mut rng = rand::thread_rng();
        rng.gen_range(min_value..=max_value)
    }

    #[test]
    fn test_upper_bound_of_pv_value() {
        let value = generate_random_pv_value();
        assert_eq!(false, (value > 1200));
    }

    #[test]
    fn test_lower_bound_of_pv_value() {
        let value = generate_random_pv_value();
        assert_eq!(true, (value > 0));
    }
}