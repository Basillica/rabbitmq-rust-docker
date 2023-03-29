use lapin::{options::*, types::FieldTable, Channel, Connection, message::BasicGetMessage, ConnectionProperties};
use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::time::{Duration};
use rand::Rng;

#[derive(Debug)]
pub struct Server {
    pub channel: Channel,
    pub consumer_tag: Option<BasicGetMessage>,
    pub connection: Connection,
    pub queue_name: String,
    pub file_name: String,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Message {
    power_value: i32,
    timestamp: i64,
}

impl Server {
    pub async fn new(rabbit_addr: &str, queue_name: &str, file_name: &str) -> Server {
        let conn = Connection::connect(rabbit_addr, ConnectionProperties::default())
            .await
            .unwrap();

        let channel = conn.create_channel()
            .await
            .unwrap();

        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();


        let consumer_tag = channel
            .basic_get(
                queue_name,
                BasicGetOptions{
                    no_ack: false,
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        Server {
            channel: channel,
            // queue: queue,
            consumer_tag: consumer_tag,
            connection: conn,
            queue_name: queue_name.to_string(),
            file_name: file_name.to_string(),
        }
    }

    pub async fn consume(&self) {
        // consumer.
        if let Some(delivery) = &self.consumer_tag {
            let payload = String::from_utf8_lossy(&delivery.data);
            println!("Received message: {}", payload);
            self.channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await
                .unwrap();

            let msg_string = String::from_utf8_lossy(&delivery.data).to_string();
            

            let msg: Message = serde_json::from_str(&msg_string).unwrap();
            let file = OpenOptions::new().create(true).append(true).open(&self.file_name);
            file
                .unwrap()
                .write(self.get_line(&msg.timestamp, &msg.power_value).as_bytes())
                .expect("write failed");

            println!("the message is {:?}", msg);

        } else {
            tokio::time::sleep(Duration::from_secs(10)).await;
        }

    }


    fn generate_random_pv_value(&self) -> u32 {
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

    pub fn get_line(&self, timestamp: &i64, power_value: &i32) -> String {
        format!("timestamp: {:?}, power_value: {:?}, pv_value: {:?} \n", timestamp, power_value, self.generate_random_pv_value() )
    }
    
}