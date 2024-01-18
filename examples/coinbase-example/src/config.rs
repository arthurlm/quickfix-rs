use std::env;

use quickfix::SessionId;

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub api_passphrase: String,
    pub api_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        fn read(key: &str) -> String {
            env::var(key).expect(&format!("Missing env variable: {key}"))
        }

        Self {
            api_key: read("COINBASE_API_KEY"),
            api_passphrase: read("COINBASE_API_PASSPHRASE"),
            api_secret: read("COINBASE_API_SECRET"),
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
