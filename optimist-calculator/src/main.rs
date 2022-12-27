use std::collections::HashMap;
use std::io;

fn main() {
    // Create a HashMap to store the wallet addresses and their values
    let mut wallets: HashMap<String, f64> = HashMap::new();

    // Loop indefinitely until the user decides to quit
    loop {
        println!("Enter a public wallet address (or 'q' to quit):");

        // Read the user's input
        let mut address = String::new();
        io::stdin().read_line(&mut address).expect("Failed to read line");
        address = address.trim().to_string();

        // Check if the user wants to quit
        if address == "q" {
            break;
        }

        // If the wallet address is not in the HashMap, ask the user for its value
        if !wallets.contains_key(&address) {
            println!("Enter the value of this wallet at its all-time high:");

            let mut value_str = String::new();
            io::stdin().read_line(&mut value_str).expect("Failed to read line");
            let value: f64 = value_str.trim().parse().expect("Failed to parse value");

            // Insert the wallet address and its value into the HashMap
            wallets.insert(address, value);
        }
    }

    // Calculate the total value of all the wallet addresses
    let total: f64 = wallets.values().sum();
    println!("Total value of all wallet addresses: {}", total);
}
