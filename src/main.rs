mod config_loader;

use std::env;
use std::time::Duration;
use reqwest::Client;
use reqwest::ClientBuilder;
use config_loader::load_config;
use tokio::time::sleep;
use futures::future::join_all;
use log::{info};
use std::sync::Once;
use warp::Filter;
use crate::config_loader::Config;

static INIT: Once = Once::new();

#[tokio::main]
async fn main() {
    INIT.call_once(|| {
        env_logger::init();
    });

    let args: Vec<String> = env::args().collect();
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    if args.contains(&"--servertest".to_string()) {
        // Define the warp filter for the /start endpoint
        let start_route = warp::path("start")
            .and(warp::post())
            .and_then(move || {
                let client = client.clone();
                async move {
                    let config = load_config("config_iter.yml");
                    run_iterations(client, config).await;
                    Ok::<_, warp::Rejection>(warp::reply::with_status("Completed iterations", warp::http::StatusCode::OK))
                }
            });

        // Start the warp server
        info!("Starting server on port 3030");
        warp::serve(start_route)
            .run(([0, 0, 0, 0], 3030))
            .await;
    } else {
        let config = load_config("config_iter.yml");
        run_iterations(client, config).await;
    }
}

async fn run_iterations(client: Client, config: Config) {
    info!("Starting run_iterations function");

    // Add a delay to ensure the server is ready
    sleep(Duration::from_secs(1)).await;
    info!("Initial delay completed");

    for i in 0..config.loop_count {
        info!("Starting loop iteration {}", i + 1);

        // Wait for the specified delay before proceeding to the next iteration
        sleep(Duration::from_millis(config.delay_between_iteration_in_ms)).await;
        info!("Delay between iterations completed");

        // Create a vector to hold all the request futures
        let mut requests = Vec::new();

        // Make an HTTP request to the start route of each replica
        for (index, address) in config.rai_service_tester.replica_address.iter().enumerate() {
            let service_name = format!("rai_service_tester_{}", index + 1);
            let address = address.replace("localhost", &service_name);
            let url = format!("{}/start", address);
            info!("Sending request to {}", url);
            let request = client.post(&url).send();
            requests.push(request);
        }

        // Wait for all requests to complete
        join_all(requests).await;
        info!("All requests for iteration {} completed", i + 1);
    }

    info!("Completed all iterations");
}