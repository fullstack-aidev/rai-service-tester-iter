use std::process::{Command, exit};
use std::thread;
use std::time::Duration;
use reqwest::blocking::Client;

pub fn main() {
    let service_name = "rai_service_tester";
    let loop_count = 2;
    let client = Client::new();
    let replica_addresses = vec![
        "http://localhost:7121/start",
        "http://localhost:7122/start",
        "http://localhost:7123/start",
    ];

    for _ in 0..loop_count {
        let status = Command::new("docker-compose")
            .args(&["up", "-d", &service_name])
            .status()
            .expect("failed to execute docker-compose");

        if !status.success() {
            eprintln!("Failed to start service {}", service_name);
            exit(1);
        }

        // Wait for 100 milliseconds before proceeding to the next iteration
        thread::sleep(Duration::from_millis(100));

        // Make an HTTP request to the start route of each replica
        for address in &replica_addresses {
            let response = client.post(address)
                .send()
                .expect("Failed to send request");

            if !response.status().is_success() {
                eprintln!("Failed to start client at {}: {:?}", address, response.status());
                exit(1);
            }
        }
    }
}