use reqwest::Client;
use serde::{Deserialize, Serialize};

// Replace YOUR_API_KEY with your actual Coinmarketcap API key
const API_KEY: &str = "YOUR-API-KEY";

#[derive(Debug, Deserialize, Serialize)]
struct Portfolio {
    id: u32,
    name: String,
    description: String,
    created_at: String,
    updated_at: String,
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    id: u32,
    symbol: String,
    name: String,
    quantity: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Cryptocurrency {
    id: u32,
    name: String,
    symbol: String,
    all_time_high: AllTimeHigh,
}

#[derive(Debug, Deserialize, Serialize)]
struct AllTimeHigh {
    price: f32,
    timestamp: u64,
}

use tokio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new HTTP client
    let client = Client::new();

    // Make a request to the Coinmarketcap API to fetch the portfolio
    let portfolio_url = format!("https://pro-api.coinmarketcap.com/v1/portfolio/{id}", id = 1);
    let response = tokio::task::block_on(client.get(&portfolio_url)
        .header("X-CMC_PRO_API_KEY", API_KEY)
        .send())?;

    // Deserialize the response into a Portfolio struct
    let portfolio: Portfolio = response.json()?;

    // Initialize a variable to hold the total value of the portfolio at all-time highs
    let mut portfolio_value_at_all_time_highs = 0.0;

    // Iterate over the entries in the portfolio
    for entry in portfolio.entries {
        // Make a request to the Coinmarketcap API to fetch the all-time high price for the token
        let cryptocurrency_url = format!("https://pro-api.coinmarketcap.com/v1/cryptocurrency/info?symbol={symbol}", symbol = entry.symbol);
        let response = tokio::task::block_on(client.get(&cryptocurrency_url)
            .header("X-CMC_PRO_API_KEY", API_KEY)
            .send())?;

        // Deserialize the response into a Cryptocurrency struct
        let cryptocurrency: Cryptocurrency = response.json()?;

        // Calculate the value of the entry at the all-time high price
        let value_at_all_time_high = cryptocurrency.all_time_high.price * entry.quantity;

        // Add the value of the entry to the total portfolio value
        portfolio_value_at_all_time_highs += value_at_all_time_high;
    }

    // Print the total portfolio value at all-time highs
    println!("Total portfolio value at all-time highs: ${:.2}", portfolio_value_at_all_time_highs);

    Ok(())
}
