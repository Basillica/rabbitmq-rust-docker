use actix_web::{get,web, App, HttpResponse, HttpServer, Responder};
use tokio::time::{self, Duration};

use rand::Rng;
use serde;
use chrono::prelude::*;
use crate::utils::server::Server;

mod utils;

const QUEUE_NAME: &'static str = "my_queue";
const RABBIT_STRING: &'static str = "amqp://localhost:5672/DSrabbitmq";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Message {
    power_value: u32,
    timestamp: i64,
}


// #[actix_web::main]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // tracing_subscriber::fmt()
    // .with_target(false)
    // .with_timer(tracing_subscriber::fmt::time::uptime())
    // .with_level(true)
    // .init();

    // let conn = Arc::new(server.connection);

    // let server_data = web::Data::new(Server {
    //     channel: server.channel,
    //     queue: server.queue,
    //     connection: server.connection,
    //     queue_name: QUEUE_NAME.to_string(),
    // });


    loop {
        let server = Server::new(RABBIT_STRING, QUEUE_NAME).await;
        println!("Service B running... {:?}", server.queue);
        server.publish().await;

        time::sleep(Duration::from_secs(5)).await;
    }
    // HttpServer::new(move || {
    //     App::new()
    //         // .app_data(conn.clone())let conn = Arc::new(server.connection);
    //         .app_data(server_data.clone())
    //         .service(hello)
    //         .service(publish)
    // })
    // .bind(("127.0.0.1", 8081))?
    // .run()
    // .await    
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


// Testing the random number generator
#[cfg(test)]
mod tests {
    use super::*;

    fn generate_random_power_value() -> u32 {
        // Get the current time
        let now = Local::now();
    
        // Calculate a multiplier based on the hour of the day
        let hour_multiplier = (now.hour() as f32 - 12.0) / 12.0;
    
        // Calculate the range of the random number based on the multiplier
        let range_size = (hour_multiplier * 45.0).abs() as u32;
        let max_value = 90;
        let min_value = if range_size > max_value { 0 } else { max_value - range_size };
    
        // Generate a random value within the range
        println!("min -> {}, max -> {}", min_value, max_value);
        let mut rng = rand::thread_rng();
        rng.gen_range(min_value..=max_value)
    }

    #[test]
    fn test_upper_bound_of_pv_value() {
        let value = generate_random_power_value();
        assert_eq!(false, (value > 90));
    }

    #[test]
    fn test_lower_bound_of_pv_value() {
        let value = generate_random_power_value();
        assert_eq!(true, (value > 0));
    }
}
