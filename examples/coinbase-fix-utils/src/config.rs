use std::env;

use quickfix::SessionId;

#[derive(Debug)]
pub struct CoinbaseConfig {
    pub api_key: String,
    pub api_passphrase: String,
    pub api_secret: String,
}

impl CoinbaseConfig {
    pub fn from_env() -> Self {
        macro_rules! read {
            ($key:expr) => {
                env::var($key).expect(concat!("Missing env variable: ", $key))
            };
        }

        Self {
            api_key: read!("COINBASE_API_KEY"),
            api_passphrase: read!("COINBASE_API_PASSPHRASE"),
            api_secret: read!("COINBASE_API_SECRET"),
        }
    }

    pub fn session_id(&self) -> SessionId {
        SessionId::try_new(
            coinbase_fix42::FIX_BEGIN_STRING,
            &self.api_key,
            "Coinbase",
            "",
        )
        .expect("Fail to build session ID")
    }
}
