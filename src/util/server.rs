use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::from_slice;
use crate::structs::nurse::Individual;

#[derive(Serialize, Deserialize)]
struct MigrationMessage {
    individuals: Vec<Individual>,
}
pub async fn run_server(port: u16) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    println!("Server listening on port {}", port);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            let msg: MigrationMessage = from_slice(&buf[..n]).unwrap();
            println!("Received {} individuals", msg.individuals.len());
            // Incorporate received individuals into the population
        });
    }
}