use coinbase_fix_utils::config::CoinbaseConfig;
use quickfix::{Dictionary, SessionSettings};

/// Minimal configuration builder to setup FIX engine.
///
/// Fill free to update it to your need.
pub fn build_session_settings(config: &CoinbaseConfig) -> anyhow::Result<SessionSettings> {
    let mut settings = SessionSettings::new();

    settings.set(None, {
        let mut params = Dictionary::new();
        params.set("ConnectionType", "initiator".to_string())?;
        params
    })?;

    settings.set(Some(config.session_id()), {
        let mut params = Dictionary::new();
        params.set("StartTime", "00:00:01".to_string())?;
        params.set("EndTime", "23:59:59".to_string())?;
        params.set("HeartBtInt", 30)?;
        params.set("SocketConnectPort", 5298)?; // ⚠️ This port should match what you have in your stunnel configuration file.
        params.set("SocketConnectHost", "127.0.0.1".to_string())?;
        params.set(
            "DataDictionary",
            "../coinbase-fix42/src/cb-FIX42-prod-sand.xml".to_string(),
        )?;
        params
    })?;

    Ok(settings)
}