extern crate solscan;

use solscan::Solscan;
use solscan::model::{Trades, Trade, MACD};

fn main() {
    // Create a new instance of the Solscan client
    let client = Solscan::new("<api_key>");

    // Connect to the user's Solscan account
    let account = client.login("<username>", "<password>").unwrap();

    // Fetch the MACD data for the desired trading pair
    let macd = client.get_macd("<trading_pair>", 100).unwrap();

    // Calculate the EMA short and EMA long values
    let ema_short = calculate_ema(macd.values, 12);
    let ema_long = calculate_ema(macd.values, 26);

    // Check for buy and sell conditions
    for i in 0..macd.values.len() {
        if ema_short[i] > ema_long[i] {
            // EMA short goes above EMA long, buy condition is met
            let trade = Trade {
                amount: 1.0,
                price: macd.close[i],
                side: "buy",
            };
            client.place_trade(account.address, trade).unwrap();
        } else if ema_short[i] < ema_long[i] {
            // EMA short goes below EMA long, sell condition is met
            let trade = Trade {
                amount: 1.0,
                price: macd.close[i],
                side: "sell",
            };
            client.place_trade(account.address, trade).unwrap();
        }
    }
}

// Calculates the exponential moving average (EMA) for the given values with the given period
fn calculate_ema(values: Vec<f64>, period: usize) -> Vec<f64> {
    let mut ema = vec![0.0; period - 1];
    let k = 2.0 / (period as f64 + 1.0);
    for i in period - 1..values.len() {
        let ema_prev = if i == period - 1 {
            values[i - 1]
        } else {
            ema[i - 1]
        };
        ema.push((values[i] - ema_prev) * k + ema_prev);
    }
    ema
}
