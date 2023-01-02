use std::io;

fn main() {
    // Read in the API key from the user
    println!("Enter your CoinMarketCap API key: ");
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).expect("Failed to read API key");

    // Read in the number of tokens in the user's portfolio
    println!("Enter the number of tokens in your portfolio: ");
    let mut num_tokens = String::new();
    io::stdin().read_line(&mut num_tokens).expect("Failed to read number of tokens");
    let num_tokens: u32 = num_tokens.trim().parse().expect("Failed to parse number of tokens");

    // Read in the information for each token
    let mut portfolio = Vec::new();
    for i in 0..num_tokens {
        println!("Enter the name of token {}: ", i + 1);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read token name");
        let name = name.trim();

        println!("Enter the number of {} in your portfolio: ", name);
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Failed to read token amount");
        let amount: u32 = amount.trim().parse().expect("Failed to parse token amount");

        portfolio.push((name, amount));
    }

    // Fetch the all time high price for each token
    let mut total_value = 0;
    for (name, amount) in &portfolio {
        let all_time_high = fetch_all_time_high(api_key, name);
        let value = all_time_high * *amount;
        total_value += value;
        println!("If {} were at their all time high, {} tokens would be worth {}", name, amount, value);
    }

    println!("Your portfolio would be worth {} if all tokens were at their all time high.", total_value);
}

// This function fetches the all time high price for a given token from the CoinMarketCap API
fn fetch_all_time_high(api_key: String, token_name: &str) -> u32 {
    // TODO: Use the CoinMarketCap API to fetch the all time high price for the given token
    // For now, we'll just return a placeholder value of 1000
    1000
}
