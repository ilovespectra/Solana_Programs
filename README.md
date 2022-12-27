_Messing around right now, currently this code returns 25 days no matter what. I have to fix it, I'll update asap._ 

Install Rust by running the following command:

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

</br>

This will download and run the Rust installation script, which will install Rust and the necessary tools on your system.

Install the solana-sdk library, which is a Rust library that provides API bindings and utilities for working with the Solana blockchain. You can install it using the following command:

```cargo install solana-sdk``` 

</b>

This will download and build the solana-sdk library, and make it available for use in your Rust projects.

If your Solana project has any additional dependencies, you can add them to your Cargo.toml file as described in the previous response. For example, to add the thread and time crates as dependencies, you would add the following lines to the [dependencies] section of your Cargo.toml file:

```
thread = "0.4.3"
time = "0.1.40"
```

</br>

For this program, your Cargo.toml should look like this:
```
[package]
name = "countdown"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
solana-sdk = "1.14.11"

```

Then, you can run the following command to install the dependencies:

```
cargo build --release 
./target/release/countdown
```

 </br>
 
This will download and build the dependencies, and make them available for use in your Rust code.
