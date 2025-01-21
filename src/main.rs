mod config_loader;

use std::time::Duration;
use reqwest::Client;
use reqwest::ClientBuilder;
use config_loader::load_config;
use tokio::time::sleep;
use futures::future::join_all;
use log::{info, error};
use std::sync::Once;

static INIT: Once = Once::new();

#[tokio::main]
async fn main() {
    INIT.call_once(|| {
        env_logger::init();
    });

    let config = load_config("config_iter.yml");

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    // Add a delay to ensure the server is ready
    sleep(Duration::from_secs(10)).await;

    for i in 0..config.loop_count {
        info!("Starting loop iteration {}", i + 1);

        // Wait for the specified delay before proceeding to the next iteration
        sleep(Duration::from_millis(config.delay_between_iteration_in_ms)).await;

        // Create a vector to hold all the request futures
        let mut requests = Vec::new();

        // Make an HTTP request to the start route of each replica
        for address in &config.rai_service_tester.replica_address {
            info!("Sending request to {}", address);
            let client = client.clone();
            let address = address.clone();
            let request = async move {
                let mut retries = 5;
                while retries > 0 {
                    let response = client.post(&format!("{}/start", address)).send().await;
                    match response {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                info!("Successfully started client at {}", address);
                                break;
                            } else {
                                error!("Failed to start client at {}: {:?} - {:?}", address, resp.status(), resp.text().await);
                            }
                        }
                        Err(err) => {
                            error!("Failed to send request to {}: {:?}", address, err);
                        }
                    }
                    retries -= 1;
                    if retries > 0 {
                        error!("Retrying in 3 seconds...");
                        sleep(Duration::from_secs(3)).await;
                    } else {
                        error!("Giving up on {}", address);
                    }
                }
            };
            requests.push(request);
        }

        // Wait for all requests to complete
        join_all(requests).await;
    }
}