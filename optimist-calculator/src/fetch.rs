extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;

const API_URL: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest";

#[derive(Deserialize)]
struct Quote {
    all_time_high: AllTimeHigh,
}

#[derive(Deserialize)]
struct AllTimeHigh {
    price: f64,
}

fn fetch_all_time_high(api_key: String, token_name: &str) -> u32 {
    let mut url = format!("{}?symbol={}", API_URL, token_name);

    let client = reqwest::Client::new();
    let mut response = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .unwrap();

    let quote: Quote = response.json().unwrap();
    quote.all_time_high.price as u32
}
