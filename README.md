## A multiservice involving a RabbitMq message broker, a consumer and a producer

### Introduction
The coding challenge was set up using Docker and docker-compose for orchestration
The programming language is Rust, and the [lapin](https://crates.io/crates/lapin) crate was used as a RabbitMq client.

### Testing the code
Due to the imcomplete nature of the code, it can only be tested at the moment by running it from the terminal.
This would imply running the command `docker compose up` to deploy the `RabbitMq` server to a docker container, and then individually running the other services from the terminal in the order.
```bash
cargo run broker
cargo run producer
cargo run consumer
```
from the respective repositories.

This will launch the three applications in the order describe above.

The consumer would then receive the message from the queue and append to a locally created file.

### Definition of services.
The `broker` acts as an orchestrator for managing the broker, to configure it and make changes to its configuration if need be.
The `producer` would be the `Meter` according to the task. It generates the messages and writes them to a queue on the broker
The `consumer` would be be the `PV Simulator`.