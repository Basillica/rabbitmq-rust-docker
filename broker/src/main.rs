use lapin::{types::FieldTable, options::QueueDeclareOptions, Connection, ConnectionProperties};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use tokio::time::{self, Duration};
use std::sync::Arc;
use serde;

const QUEUE_NAME: &'static str = "my_queue";


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Message {
    id: u32,
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
    .with_target(false)
    .with_timer(tracing_subscriber::fmt::time::uptime())
    .with_level(true)
    .init();

    // "amqp://localhost:5672"
    // kinda workds "amqp://guest:guest@rabbitmq/"
    let conn = Connection::connect("amqp://localhost:5672/DSrabbitmq", ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    let queue = channel
        .queue_declare(
            QUEUE_NAME,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Service A running... {:?}", queue.name());

    loop {
        time::sleep(Duration::from_secs(1)).await;
    }
}


// #[actix_web::main]
// async fn main() -> Result<(), std::io::Error> {
//     tracing_subscriber::fmt()
//     .with_target(false)
//     .with_timer(tracing_subscriber::fmt::time::uptime())
//     .with_level(true)
//     .init();

//     let conn = Connection::connect("amqp://localhost:5672", ConnectionProperties::default()).await.unwrap();
//     let channel = conn.create_channel().await.unwrap();
//     let queue = channel
//         .queue_declare(
//             QUEUE_NAME,
//             QueueDeclareOptions::default(),
//             FieldTable::default(),
//         )
//         .await.unwrap();
    
//     println!("Service A running... {:?}", queue.name());

//     let conn = Arc::new(conn);
//     HttpServer::new(move || {
//         App::new()
//             .app_data(conn.clone())
//             .service(hello)
//             .service(echo)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await

// }

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// docker exec rabbitmq rabbitmqctl list_vhosts
// curl -i -u guest:guest http://localhost:15672/api/vhosts