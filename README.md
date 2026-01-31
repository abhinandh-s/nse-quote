# nse-quote

An async, strongly-typed Rust client for fetching equity quote data from the NSE (National Stock Exchange of India).

This crate wraps the unofficial NSE equity quote API and provides Typed Data.


> [!WARNING]
> This uses an undocumented NSE endpoint. The API may change or break without notice.
> Therefore, No plan to publish in `crates.io`

---

## Features

- Async-first design
- Safe error handling
- Typed Data
- No panics, No unsafe

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nse-quote = { git = "https://github.com/abhinandh-s/nse-quote", version = "0.1.100" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Examples 

```bash 
cargo run --example quote
cargo run --example preopen
cargo run --example price_only

```

## Disclaimer

This project is not affiliated with NSE.
Use responsibly and respect NSE’s terms of use.
