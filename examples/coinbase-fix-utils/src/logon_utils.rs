use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use coinbase_fix42_order_entry::{
    field_id::{self, PASSWORD},
    Logon,
};
use coinbase_fix50_market_data::field_id::USERNAME;
use hmac::{Hmac, Mac};
use quickfix::*;
use sha2::Sha256;

use crate::config::CoinbaseConfig;

pub fn fill_message(msg: &mut Message, config: &CoinbaseConfig) -> Result<(), QuickFixError> {
    // Set password
    msg.set_field(USERNAME, "s.nakamoto")
        .expect("Fail to set password");
    msg.set_field(PASSWORD, config.api_passphrase.as_str())
        .expect("Fail to set password");

    Ok(())
}

/// Add signature to a `Logon` message.
///
/// This function is a direct implementation of Coinbase signature spec.
/// See: <https://docs.cloud.coinbase.com/exchange/docs/fix-msg-order-entry#logon-a>
pub fn sign(msg: &mut Message, config: &CoinbaseConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Add few macro to make it easier to work with `Message`.
    macro_rules! read_header {
        ($tag:expr) => {
            msg.with_header(|h| h.get_field($tag))
                .expect("Missing mandatory message header")
        };
    }

    // Build pre-sign message by extracting everything from auto-generated FIX message.
    let pre_sign = [
        &read_header!(field_id::SENDING_TIME),
        Logon::MSG_TYPE_BYTES,
        &read_header!(field_id::MSG_SEQ_NUM),
        &config.api_key,
        "Coinbase",
        &config.api_passphrase,
    ]
    .join("\x01");

    // Generate signature.
    let secret = BASE64.decode(config.api_secret.as_bytes())?;
    let mut mac = Hmac::<Sha256>::new_from_slice(&secret)?;
    mac.update(pre_sign.as_bytes());
    let signature_raw = mac.finalize();
    let signature = BASE64.encode(signature_raw.into_bytes());

    // Append it to outgoing message.
    msg.set_field(field_id::RAW_DATA_LENGTH, signature.len())?;
    msg.set_field(field_id::RAW_DATA, signature)?;

    Ok(())
}
