use std::{thread, time::Duration};
use reqwest::Client;

const HALVING_INTERVAL: u64 = 52_592_000; // seconds
const HALVING_BLOCKS: u64 = 21_600_000; // blocks
const API_URL: &str = "https://api.solana.com";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(API_URL)
        .header("Content-Type", "application/json")
        .send()?;
    let json: serde_json::Value = response.json()?;
    let current_block_height: u64 = json["result"]["block_height"].as_u64()
        .expect("Error parsing block height");

    let mut blocks_left = HALVING_BLOCKS;
    let mut halving_block_height = 0;

    // Find the block height of the previous halving
    while blocks_left > 0 {
        blocks_left -= HALVING_BLOCKS;
        halving_block_height += HALVING_BLOCKS;
    }

    // Calculate the blocks remaining until the next halving
    blocks_left = current_block_height - halving_block_height;

    loop {
        let seconds_left = (blocks_left as f64 / 10.0).ceil() as u64;
        let days_left = seconds_left / 86400;
        let hours_left = (seconds_left % 86400) / 3600;
        let minutes_left = (seconds_left % 3600) / 60;
        let seconds_left = seconds_left % 60;

        println!(
            "{} days, {} hours, {} minutes, and {} seconds until the next halving",
            days_left, hours_left, minutes_left, seconds_left
        );

        thread::sleep(Duration::from_secs(HALVING_INTERVAL));
        blocks_left -= HALVING_BLOCKS;
    }
}
