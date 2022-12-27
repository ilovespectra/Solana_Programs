extern crate gtk;
extern crate reqwest;

use gtk::prelude::*;
use reqwest::Client;

fn main() {
    // Initialize GTK and create the main window
    gtk::init().expect("Failed to initialize GTK.");
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    // Set up the UI elements
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let entry = gtk::Entry::new();
    let button = gtk::Button::new_with_label("Calculate Totals");
    let label = gtk::Label::new(None);
    vbox.pack_start(&entry, false, false, 0);
    vbox.pack_start(&button, false, false, 0);
    vbox.pack_start(&label, false, false, 0);
    window.add(&vbox);

    // Set up the button click event handler
    let client = Client::new();
    let label_clone = label.clone();
    button.connect_clicked(move |_| {
        let wallet_addresses = entry.get_text().unwrap();
        let totals = calculate_totals(&client, &wallet_addresses);
        label_clone.set_text(&format!("Total: {}", totals));
    });

    // Set up the window close event handler
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show the window and run the GTK event loop
    window.show_all();
    gtk::main();
}

fn calculate_totals(client: &Client, wallet_addresses: &str) -> f64 {
    let mut totals = 0.0;

    // Split the wallet addresses by newline
    for address in wallet_addresses.split('\n') {
        // Look up the token balance for the address
        let balance = get_token_balance(client, address);

        // Look up the all-time high price for the token
        let price = get_token_price(client, address);
        
        // Calculate the total value of the token balance at the all-time high price
      totals += balance * price;
    }

    totals
}

fn get_token_balance(client: &Client, address: &str) -> f64 {
    // Use the reqwest library to make an HTTP request to a cryptocurrency balance API
    // and retrieve the token balance for the given address
    let balance: f64 = client
        .get(format!("balance_api_url/{}", address).as_str())
        .send()
        .unwrap()
        .json()
        .unwrap();

    balance
}

fn get_token_price(client: &Client, address: &str) -> f64 {
    // Use the reqwest library to make an HTTP request to a cryptocurrency price API
    // and retrieve the all-time high price for the token associated with the given address
    let price: f64 = client
        .get(format!("price_api_url/{}", address).as_str())
        .send()
        .unwrap()
        .json()
        .unwrap();

    price
}
