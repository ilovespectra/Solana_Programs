This has turned into just a general rust repo, not worth cloning ATM, public to get help finishing these programs.

<i>Most of these programs still receive plenty of errors, I'm writing these with the help of the chat.openai.com bot, who has been incredibly helpful in drafting and debugging these codes to get them to this point. Feel free to make any corrections you can find, fork, PR, or just clone and see what we can come up with.</i>

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

Then, you can run the following command to install the dependencies:

```
cargo build
```

 </br>
 
This will download and build the dependencies, and make them available for use in your Rust code.
