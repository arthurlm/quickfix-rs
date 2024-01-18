# Examples

Most example here are made for Coinbase market.

_Why ?_

Because it is the simplest one to setup and it has a sandbox to play with.

## Getting started

1. Install [`stunnel`](https://www.stunnel.org/).
2. Run it with `stunnel ./examples/stunnel.conf`.
3. Create a API token to [Coinbase sandbox API](https://public.sandbox.exchange.coinbase.com/profile/api) with:
   - Trade permission
   - Your IP
4. Setup following env variables:
   - `COINBASE_API_KEY`
   - `COINBASE_API_PASSPHRASE`
   - `COINBASE_API_SECRET`
5. Check the code and update to your need.
6. Launch the `coinbase-example` app 🚀 !!!
