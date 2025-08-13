use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

use coinbase_example::*;
use coinbase_fix42_order_entry::{
    field_types::{MsgType, OrdStatus, OrdType, SelfTradePrevention, Side, TimeInForce},
    *,
};
use coinbase_fix_utils::{config::CoinbaseConfig, logon_utils};
use parking_lot::RwLock;
use quickfix::*;
use uuid::Uuid;

/// Main FIX application.
///
/// It is here where callback will be implemented and "state" will be handled.
///
/// NOTE: keep in mind, this example is minimal. In real world application using a message bus (with `std::mpsc`) may
///       avoid the app to growth to much.
struct MyApplication {
    config: CoinbaseConfig,
    shared_state: RwLock<AppSharedState>,
}

#[derive(Default)]
struct AppSharedState {
    /// Sent order that are not in a final state.
    pending_orders_ids: HashSet<Uuid>,
    /// Map of "Order ID" to "Client Order ID".
    ///
    /// Coinbase sent "Order ID" to "Client Order ID" only once when order is in "New" state.
    market_order_mapping: HashMap<Uuid, Uuid>,
}

impl MyApplication {
    fn from_env() -> Self {
        Self {
            config: CoinbaseConfig::from_env(),
            shared_state: RwLock::new(AppSharedState::default()),
        }
    }

    fn add_order(&self, client_order_id: Uuid) {
        self.shared_state
            .write()
            .pending_orders_ids
            .insert(client_order_id);
    }

    fn set_order_mapping(&self, client_order_id: Uuid, market_order_id: Uuid) {
        self.shared_state
            .write()
            .market_order_mapping
            .insert(market_order_id, client_order_id);
    }

    fn remove_order(&self, market_order_id: Uuid) {
        let mut state = self.shared_state.write();

        if let Some(client_order_id) = state.market_order_mapping.get(&market_order_id).copied() {
            state.pending_orders_ids.remove(&client_order_id);
        }
    }

    fn is_order_pending_list_empty(&self) -> bool {
        self.shared_state.read().pending_orders_ids.is_empty()
    }
}

impl ApplicationCallback for MyApplication {
    fn on_msg_to_admin(&self, msg: &mut Message, _session: &SessionId) {
        // Intercept login message automatically sent by quickfix library
        let msg_type = msg
            .with_header(|h| h.get_field(field_id::MSG_TYPE))
            .and_then(|x| MsgType::from_const_bytes(x.as_bytes()).ok());

        if msg_type == Some(MsgType::Logon) {
            // Complete missing required fields.
            logon_utils::fill_message(msg, &self.config).expect("Fail to complete logon message");

            // Sign message.
            logon_utils::sign(msg, &self.config).expect("Fail to sign logon message");
        }

        // print_decoded_msg("TO ADMIN", msg);
    }

    // fn on_msg_to_app(&self, msg: &mut Message, _session: &SessionId) -> Result<(), MsgToAppError> {
    //     print_decoded_msg("TO APP", msg);
    //     Ok(())
    // }

    // fn on_msg_from_admin(
    //     &self,
    //     msg: &Message,
    //     _session: &SessionId,
    // ) -> Result<(), MsgFromAdminError> {
    //     print_decoded_msg("FROM ADMIN", msg);
    //     Ok(())
    // }

    fn on_msg_from_app(&self, msg: &Message, _session: &SessionId) -> Result<(), MsgFromAppError> {
        println!();
        println!("==== FROM APP =====");
        match Messages::decode(msg.clone()) {
            Ok(Messages::ExecutionReport(x)) => {
                // Read some fields from struct
                let market_order_id: Uuid = x.get_order_id().parse().expect("Invalid order ID");
                let client_order_id: Option<Uuid> = x.get_cl_ord_id().and_then(|x| x.parse().ok());

                // Print some information about the execution report.
                println!("Received execution report with:");
                println!("- Order ID:           {market_order_id}");
                println!("- Client Order ID:    {client_order_id:?}");
                println!("- Symbol:             {}", x.get_symbol());
                println!("- Status:             {:?}", x.get_ord_status());
                if x.get_ord_status() == OrdStatus::Rejected {
                    println!("- Reject reason:      {:?}", x.get_ord_rej_reason());
                }

                // Update our application state.
                if let Some(client_order_id) = client_order_id {
                    self.set_order_mapping(client_order_id, market_order_id);
                }

                if is_order_completed(x.get_ord_status()) {
                    self.remove_order(market_order_id);
                }
            }
            Ok(msg) => println!("{msg:?}"),
            Err(err) => eprintln!("Cannot decode message: {err:?}"),
        }
        println!("===================");

        Ok(())
    }
}

#[allow(dead_code)]
fn print_decoded_msg(tag: &str, msg: &Message) {
    println!();
    println!("==== {tag} =====");
    match Messages::decode(msg.clone()) {
        Ok(msg) => println!("{msg:?}"),
        Err(err) => eprintln!("Cannot decode message: {err:?}"),
    }
    println!("===================");
}

fn main() -> anyhow::Result<()> {
    // Init our callbacks.
    let my_app = MyApplication::from_env();

    // Init FIX engine.
    let settings = build_session_settings(&my_app.config)?;
    let store_factory = MemoryMessageStoreFactory::new(); // Coinbase do not have FIX replay enabled.
    let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;
    let app = Application::try_new(&my_app)?;

    let mut acceptor = Initiator::try_new(
        &settings,
        &app,
        &store_factory,
        &log_factory,
        FixSocketServerKind::SingleThreaded,
    )?;

    // Start the engine.
    println!(">> Starting FIX engine 🚀");
    acceptor.start()?;

    // Wait for login sequence completion
    while !acceptor.is_logged_on()? {
        thread::sleep(Duration::from_millis(250));
    }
    println!(">> We are now logged on. Let's trade 🎇");

    // Send new order
    let order = new_order(&my_app)?;
    let session_id = my_app.config.order_entry_session_id();

    println!(">> Sending order 💸");
    send_to_target(order.into(), &session_id)?;

    // Wait until order is filled
    println!(">> Waiting for order completion ⏳");
    while !my_app.is_order_pending_list_empty() {
        thread::sleep(Duration::from_millis(250));
    }

    // Stop and close everything.
    println!(">> Stopping FIX engine 👋");
    acceptor.stop()?;
    Ok(())
}

fn new_order(app: &MyApplication) -> anyhow::Result<NewOrderSingle> {
    let cl_ord_id = Uuid::new_v4();
    let symbol = "BTC-USD".to_string();
    let transact_time = "foo".to_string();

    // Build the order message.
    let mut order = NewOrderSingle::try_new(
        cl_ord_id.to_string(),
        symbol,
        Side::Buy,
        transact_time,
        OrdType::Market,
    )?;

    order.set_order_qty(0.001)?;
    order.set_self_trade_prevention(SelfTradePrevention::CancelNewest)?;
    order.set_time_in_force(TimeInForce::Day)?;

    // Register it in our application.
    app.add_order(cl_ord_id);

    Ok(order)
}

fn is_order_completed(status: OrdStatus) -> bool {
    matches!(
        status,
        OrdStatus::Filled
            | OrdStatus::DoneForDay
            | OrdStatus::Canceled
            | OrdStatus::Replaced
            | OrdStatus::Stopped
            | OrdStatus::Rejected
            | OrdStatus::Suspended
            | OrdStatus::Calculated
            | OrdStatus::Expired
            | OrdStatus::AcceptedForBidding
            | OrdStatus::PendingReplace
    )
}
