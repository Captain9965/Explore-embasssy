# EMBASSY:
To use embassy, you will first need to install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), (Rust's package manager).
To download [embassy](https://embassy.dev/book/dev/getting_started.html),run the following commands on the shell:
```
    git clone https://github.com/embassy-rs/embassy.git
    cd embassy
    git submodule update --init
```
Make sure the configurations in the `Cargo.toml` and `Config.toml` file are as per your target, in this case, it is the STM32F103C8.
To compile:
```
    cargo run --bin embassy-demo --release
```
You should see debug statements if you have a probe linked to the target, any SWD/JTAG probe would do.