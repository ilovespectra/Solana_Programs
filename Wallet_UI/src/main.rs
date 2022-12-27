extern crate solana_sdk;

use solana_sdk::{
    account_info::AccountInfo,
    account_utils::State,
    client::SyncClient,
    commitment_config::CommitmentConfig,
    native_token::sol_to_lamports,
    pubkey::Pubkey,
};
use std::{env, process};

fn main() {
    let mut args = env::args();
    let _ = args.next(); // skip the first argument (the program name)

    if let Some(url) = args.next() {
        let client = SyncClient::new(url);
        let commitment = CommitmentConfig::default();

        println!("Enter your account ID: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let account_id = match Pubkey::from_str(&input.trim()) {
            Ok(account_id) => account_id,
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        };

        let account_info = match client.get_account_info(&account_id, commitment) {
            Ok(account_info) => account_info,
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        };

        let balance = match account_info.lamports {
            Some(lamports) => sol_to_lamports(lamports as f64),
            None => {
                eprintln!("Error: unable to retrieve account balance");
                process::exit(1);
            }
        };

        println!("Account balance: {} SOL", balance);
    } else {
        eprintln!("Error: missing argument: url");
        process::exit(1);
    }
}
