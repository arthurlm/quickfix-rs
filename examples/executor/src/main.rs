use std::{
    env,
    io::{stdin, Read},
    process::exit,
    sync::atomic::{AtomicU32, Ordering},
};

use quickfix::*;
use thiserror::Error;

#[derive(Debug, Error)]
enum ExecutorError {
    #[error("quickfix: {0}")]
    QuickFix(#[from] QuickFixError),

    #[error("Missing field: id={0}")]
    MissingField(i32),

    #[error("invalid message type")]
    InvalidMessageType,

    #[error("invalid order type")]
    InvalidOrderType,
}

struct FixExecutor {
    last_order_id: AtomicU32,
    last_exec_id: AtomicU32,
}

impl Default for FixExecutor {
    fn default() -> Self {
        Self {
            last_order_id: AtomicU32::new(1),
            last_exec_id: AtomicU32::new(1),
        }
    }
}

impl FixExecutor {
    fn generate_execution_report(&self, msg: &Message) -> Result<Message, ExecutorError> {
        use quickfix_msg40::field_id as fix40_id;
        use quickfix_msg40::field_types::{ExecTransType, MsgType, OrdStatus, OrdType};
        use quickfix_msg41::field_id as fix41_id;
        use quickfix_msg41::field_types::ExecType;

        macro_rules! get_header {
            ($id:expr) => {
                msg.with_header(|h| h.get_field($id))
                    .ok_or(ExecutorError::MissingField($id))
            };
        }

        macro_rules! get_field {
            ($id:expr) => {
                msg.get_field($id).ok_or(ExecutorError::MissingField($id))
            };
        }

        let msg_type = MsgType::from_const_bytes(get_header!(fix40_id::MSG_TYPE)?.as_bytes());
        if msg_type != Ok(MsgType::NewOrderSingle) {
            return Err(ExecutorError::InvalidMessageType);
        }

        let ord_type = OrdType::from_const_bytes(get_field!(fix40_id::ORD_TYPE)?.as_bytes());
        if ord_type != Ok(OrdType::Limit) {
            return Err(ExecutorError::InvalidOrderType);
        }

        let begin_string = get_header!(fix40_id::BEGIN_STRING)?;
        let symbol = get_field!(fix40_id::SYMBOL)?;
        let side = get_field!(fix40_id::SIDE)?;
        let order_qty = get_field!(fix40_id::ORDER_QTY)?;
        let price = get_field!(fix40_id::PRICE)?;
        let cl_ord_id = get_field!(fix40_id::CL_ORD_ID)?;

        let mut execution_report = Message::new();
        execution_report.with_header_mut(|h| {
            h.set_field(fix40_id::BEGIN_STRING, begin_string.as_str())?;
            h.set_field(fix40_id::MSG_TYPE, MsgType::ExecutionReport)?;

            Ok::<_, QuickFixError>(())
        })?;

        execution_report.set_field(fix40_id::ORDER_ID, self.gen_order_id())?;
        execution_report.set_field(fix40_id::EXEC_ID, self.gen_exec_id())?;
        execution_report.set_field(fix40_id::ORD_STATUS, OrdStatus::Filled)?;
        execution_report.set_field(fix40_id::SYMBOL, symbol)?;
        execution_report.set_field(fix40_id::SIDE, side)?;
        execution_report.set_field(fix40_id::CUM_QTY, order_qty.as_str())?;
        execution_report.set_field(fix40_id::AVG_PX, price.as_str())?;
        execution_report.set_field(fix40_id::LAST_SHARES, order_qty.as_str())?;
        execution_report.set_field(fix40_id::LAST_PX, price)?;
        execution_report.set_field(fix40_id::CL_ORD_ID, cl_ord_id)?;
        execution_report.set_field(fix40_id::ORDER_QTY, order_qty)?;

        if begin_string == quickfix_msg40::FIX_BEGIN_STRING
            || begin_string == quickfix_msg41::FIX_BEGIN_STRING
            || begin_string == quickfix_msg42::FIX_BEGIN_STRING
        {
            execution_report.set_field(fix40_id::EXEC_TRANS_TYPE, ExecTransType::New)?;
        }

        if begin_string.as_str() >= quickfix_msg41::FIX_BEGIN_STRING {
            execution_report.set_field(fix41_id::EXEC_TYPE, ExecType::DoneForDay)?; // Fill is not present in every FIX version (ex: 4.4)
            execution_report.set_field(fix41_id::LEAVES_QTY, 0)?;
        }

        Ok(execution_report)
    }

    fn gen_order_id(&self) -> u32 {
        self.last_order_id.fetch_add(1, Ordering::Relaxed)
    }

    fn gen_exec_id(&self) -> u32 {
        self.last_exec_id.fetch_add(1, Ordering::Relaxed)
    }
}

impl ApplicationCallback for FixExecutor {
    fn on_msg_from_app(
        &self,
        msg: &Message,
        session_id: &SessionId,
    ) -> Result<(), MsgFromAppError> {
        let execution_report = self
            .generate_execution_report(msg)
            .map_err(|_err| MsgFromAppError::IncorrectDataFormat)?;

        send_to_target(execution_report, session_id).expect("Fail to send message to target");

        Ok(())
    }
}

fn main() -> Result<(), QuickFixError> {
    let args: Vec<_> = env::args().collect();
    let Some(config_file) = args.get(1) else {
        eprintln!("Bad program usage: {} <config_file>", args[0]);
        exit(1);
    };

    let executor = FixExecutor::default();

    let settings = SessionSettings::try_from_path(config_file)?;
    let store_factory = FileMessageStoreFactory::try_new(&settings)?;
    let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;
    let app = Application::try_new(&executor)?;

    let mut acceptor = Acceptor::try_new(
        &settings,
        &app,
        &store_factory,
        &log_factory,
        FixSocketServerKind::SingleThreaded,
    )?;
    acceptor.start()?;

    println!(">> App running, press 'q' to quit");
    let mut stdin = stdin().lock();
    let mut stdin_buf = [0];
    loop {
        let _ = stdin.read_exact(&mut stdin_buf);
        if stdin_buf[0] == b'q' {
            break;
        }
    }

    acceptor.stop()?;
    Ok(())
}
