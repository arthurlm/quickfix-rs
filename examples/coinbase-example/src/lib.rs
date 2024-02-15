use coinbase_fix_utils::config::CoinbaseConfig;
use quickfix::{Dictionary, SessionSettings};

/// Minimal configuration builder to setup FIX engine.
///
/// Fill free to update it to your need.
pub fn build_session_settings(config: &CoinbaseConfig) -> anyhow::Result<SessionSettings> {
    let mut settings = SessionSettings::new();

    settings.set(None, {
        let mut params = Dictionary::new();
        params.set("ConnectionType", "initiator")?;
        params
    })?;

    settings.set(Some(&config.order_entry_session_id()), {
        let mut params = Dictionary::new();
        params.set("StartTime", "00:00:01")?;
        params.set("EndTime", "23:59:59")?;
        params.set("HeartBtInt", 30)?;
        params.set("SocketConnectPort", 5298)?; // ⚠️ This port should match what you have in your stunnel configuration file.
        params.set("SocketConnectHost", "127.0.0.1")?;
        params.set("DataDictionary", "data/order-entry/FIX42-prod-sand.xml")?;
        params
    })?;

    settings.set(Some(&config.market_data_session_id()), {
        let mut params = Dictionary::new();
        params.set("StartTime", "00:00:01")?;
        params.set("EndTime", "23:59:59")?;
        params.set("HeartBtInt", 30)?;
        params.set("SocketConnectPort", 7221)?; // ⚠️ This port should match what you have in your stunnel configuration file.
        params.set("SocketConnectHost", "127.0.0.1")?;
        params.set("DefaultApplVerID", "9" /* FIX 5.0 SP2 */)?;
        params.set("DataDictionary", "data/market-data/FIX50-prod-sand.xml")?;
        params.set(
            "TransportDataDictionary",
            "data/market-data/FIXT11-prod-sand.xml",
        )?;
        params
    })?;

    Ok(settings)
}
