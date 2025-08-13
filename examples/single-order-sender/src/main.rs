use std::{collections::HashSet, env, process::exit, thread, time::Duration};

use parking_lot::RwLock;
use quickfix::*;
use quickfix_msg44::{
    field_types::{OrdStatus, OrdType, Side},
    Messages, NewOrderSingle,
};
use uuid::Uuid;

#[derive(Default)]
struct SingleOrderSender {
    pending_orders_ids: RwLock<HashSet<Uuid>>,
}

impl SingleOrderSender {
    fn add_order(&self, order_id: Uuid) {
        self.pending_orders_ids.write().insert(order_id);
    }

    fn remove_order(&self, order_id: Uuid) {
        self.pending_orders_ids.write().remove(&order_id);
    }

    fn is_order_pending_list_empty(&self) -> bool {
        self.pending_orders_ids.read().is_empty()
    }
}

impl ApplicationCallback for SingleOrderSender {
    fn on_msg_from_app(&self, msg: &Message, _session: &SessionId) -> Result<(), MsgFromAppError> {
        println!();
        println!("==== FROM APP =====");
        match Messages::decode(msg.clone()) {
            Ok(Messages::ExecutionReport(x)) => {
                // Read some fields from struct
                let market_order_id = x.get_order_id();
                let client_order_id: Uuid = x
                    .get_cl_ord_id()
                    .and_then(|x| x.parse().ok())
                    .expect("Missing client order ID");

                // Print some information about the execution report.
                println!("Received execution report with:");
                println!("- Order ID:           {market_order_id}");
                println!("- Client Order ID:    {client_order_id:?}");
                println!("- Symbol:             {:?}", x.get_symbol());
                println!("- Status:             {:?}", x.get_ord_status());
                if x.get_ord_status() == OrdStatus::Rejected {
                    println!("- Reject reason:      {:?}", x.get_ord_rej_reason());
                }

                // Update our application state.
                if x.get_ord_status() == OrdStatus::Filled {
                    self.remove_order(client_order_id);
                }
            }
            Ok(msg) => println!("{msg:?}"),
            Err(err) => eprintln!("Cannot decode message: {err:?}"),
        }
        println!("===================");

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    let Some(config_file) = args.get(1) else {
        eprintln!("Bad program usage: {} <config_file>", args[0]);
        exit(1);
    };

    // Init our callbacks.
    let single_order_sender = SingleOrderSender::default();

    // Init FIX engine.
    let settings = SessionSettings::try_from_path(config_file)?;
    let store_factory = FileMessageStoreFactory::try_new(&settings)?;
    let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;
    let app = Application::try_new(&single_order_sender)?;

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
    let order = new_order(&single_order_sender)?;
    let session_id = SessionId::try_new("FIX.4.4", "CLIENT1", "SERVER1", "")?;

    println!(">> Sending order 💸");
    send_to_target(order.into(), &session_id)?;

    // Wait until order is filled
    println!(">> Waiting for order completion ⏳");
    while !single_order_sender.is_order_pending_list_empty() {
        thread::sleep(Duration::from_millis(250));
    }

    // Stop and close everything.
    println!(">> Stopping FIX engine 👋");
    acceptor.stop()?;
    Ok(())
}

fn new_order(app: &SingleOrderSender) -> anyhow::Result<NewOrderSingle> {
    let cl_ord_id = Uuid::new_v4();

    // Build the order message.
    let mut order = NewOrderSingle::try_new(
        cl_ord_id.to_string(),
        Side::Buy,
        build_transact_time(),
        OrdType::Limit,
    )?;

    order.set_order_qty(14.0)?;
    order.set_symbol("APPL US Equity".to_string())?;
    order.set_price(893.123)?;

    // Register it in our application.
    app.add_order(cl_ord_id);

    Ok(order)
}

fn build_transact_time() -> String {
    let now = chrono::Utc::now();
    now.format("%Y%m%d-%T%.3f").to_string()
}
