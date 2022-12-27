This program uses the thread and time crates to display a countdown timer for the next Solana halving. It calculates the number of blocks remaining until the next halving and converts that to the number of seconds remaining. It then displays the remaining time in days, hours, minutes, and seconds.

To use this program, you will need to have the thread and time crates installed. For this program, your Cargo.toml should look like this:

```
[package]
name = "countdown"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
solana-sdk = "1.14.11"
```
Then, you can compile and run the program using the following commands:

```
cargo build --release
./target/release/countdown
```
</br>
This will start the countdown timer, which will update every HALVING_INTERVAL seconds (which is currently set to 52,592,000 seconds, or approximately 1 year and 4 months). The countdown will continue indefinitely until the program is stopped.
