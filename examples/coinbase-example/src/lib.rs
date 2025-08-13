use coinbase_fix_utils::config::CoinbaseConfig;
use quickfix::{dictionary_item::*, Dictionary, SessionSettings};

/// Minimal configuration builder to setup FIX engine.
///
/// Fill free to update it to your need.
pub fn build_session_settings(config: &CoinbaseConfig) -> anyhow::Result<SessionSettings> {
    let mut settings = SessionSettings::new();

    settings.set(
        None,
        Dictionary::try_from_items(&[&ConnectionType::Initiator])?,
    )?;

    settings.set(
        Some(&config.order_entry_session_id()),
        Dictionary::try_from_items(&[
            &StartTime("00:00:01"),
            &EndTime("23:59:59"),
            &HeartBtInt(30),
            &SocketConnectPort(4198),
            &SocketConnectHost("fix-public.sandbox.exchange.coinbase.com"),
            &DataDictionary("data/order-entry/FIX42-prod-sand.xml"),
        ])?,
    )?;

    settings.set(
        Some(&config.market_data_session_id()),
        Dictionary::try_from_items(&[
            &StartTime("00:00:01"),
            &EndTime("23:59:59"),
            &HeartBtInt(30),
            &SocketConnectPort(6121),
            &SocketConnectHost("fix-md.sandbox.exchange.coinbase.com"),
            &DataDictionary("data/order-entry/FIX42-prod-sand.xml"),
            &DefaultApplVerID("9" /* FIX 5.0 SP2 */),
            &DataDictionary("data/market-data/FIX50-prod-sand.xml"),
            &TransportDataDictionary("data/market-data/FIXT11-prod-sand.xml"),
        ])?,
    )?;

    Ok(settings)
}
