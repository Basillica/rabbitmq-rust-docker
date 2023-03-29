use lapin::{options::*, types::FieldTable, Channel, Queue, Connection, BasicProperties, ConnectionProperties};
use chrono::prelude::*;
use actix_web::{ HttpResponse };

use rand::Rng;
use serde_json::{to_string};



#[derive(Debug)]
pub struct Server {
    pub channel: Channel,
    pub queue: Queue,
    pub connection: Connection,
    pub queue_name: String,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Message {
    power_value: u32,
    timestamp: i64,
}

impl Server {
    pub async fn new(rabbit_addr: &str, queue_name: &str) -> Server {
        let conn = Connection::connect(rabbit_addr, ConnectionProperties::default())
        .await
        .unwrap();
        let channel = conn.create_channel()
            .await
            .unwrap();
        let queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        Server {
            channel: channel,
            queue: queue,
            connection: conn,
            queue_name: queue_name.to_string(),
        }
    }

    pub async fn publish(&self) -> HttpResponse {
        // loop
        let now = Utc::now();
        let msg = Message {
            power_value: self.generate_random_power_value(),
            timestamp: now.timestamp(),
        };
        
        let msg_string = to_string(&msg).unwrap();
    
        println!("publishing message {}", msg_string);
    
        self.channel
            .basic_publish(
                "",
                &self.queue_name,
                BasicPublishOptions::default(),
                msg_string.as_bytes(),//.to_vec(),
                BasicProperties::default(),
            )
            .await
            .unwrap();

        HttpResponse::Ok().body("Message published to queue!")
        // end of loop
    }


    fn generate_random_power_value(&self) -> u32 {
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

}