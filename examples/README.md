# Examples

Most example here are made for Coinbase market.

_Why ?_

Because it is the simplest one to setup and it has a sandbox to play with.

## Getting started

1. Create a API token to [Coinbase sandbox API](https://public.sandbox.exchange.coinbase.com/profile/api) with:
   - Trade permission
   - Your IP
2. Setup following env variables:
   - `COINBASE_API_KEY`
   - `COINBASE_API_PASSPHRASE`
   - `COINBASE_API_SECRET`
3. Check the code and update to your need.
4. Add some fund to your [sandbox portfolio](https://public.sandbox.exchange.coinbase.com/portfolios).
5. Launch the `coinbase-example` app 🚀 !!!

## Including code from github instead of crates.io

That is so simple. Just add following lines to your `Cargo.toml` file:

```toml
coinbase-fix42-order-entry = { git = "https://github.com/arthurlm/quickfix-rs.git" }
coinbase-fix50-market-data = { git = "https://github.com/arthurlm/quickfix-rs.git" }
coinbase-fix-utils = { git = "https://github.com/arthurlm/quickfix-rs.git" }
quickfix = { git = "https://github.com/arthurlm/quickfix-rs.git" }
```

## Other examples

**NOTE**: Make sure to clean FIX file message store before running examples.

Running executor (simple app that execute every order you sent to it):

```sh
cargo r --bin=executor examples/configs/server.ini
```

Running single order sender:

```sh
cargo r --bin=single-order-sender  examples/configs/client.ini
```
