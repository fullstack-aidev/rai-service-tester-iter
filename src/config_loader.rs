use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use serde_yaml;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub loop_count: usize,
    pub delay_between_iteration_in_ms: u64,
    pub rai_service_tester: RaiServiceTester,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RaiServiceTester {
    pub replica_address: Vec<String>,
}

pub fn load_config(file_path: &str) -> Config {
    let mut file = File::open(file_path).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config file");
    serde_yaml::from_str(&contents).expect("Failed to parse config file")
}