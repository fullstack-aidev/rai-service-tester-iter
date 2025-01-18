mod config_loader;

use std::thread;
use std::time::Duration;
use reqwest::blocking::Client;
use config_loader::load_config;

fn main() {
    let config = load_config("config_iter.yml");

    let client = Client::new();

    for _ in 0..config.loop_count {
        // Wait for the specified delay before proceeding to the next iteration
        thread::sleep(Duration::from_millis(config.delay_between_iteration_in_ms));

        // Make an HTTP request to the start route of each replica
        for address in &config.rai_service_tester.replica_address {
            let response = client.post(&format!("{}/start", address))
                .send()
                .expect("Failed to send request");

            if !response.status().is_success() {
                eprintln!("Failed to start client at {}: {:?}", address, response.status());
                std::process::exit(1);
            }
        }
    }
}