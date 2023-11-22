use std::{error::Error, fmt, str::FromStr};

use quickfix::{FieldMap, Message, SessionId};

#[derive(Debug)]
#[non_exhaustive]
pub enum BadCommand {
    Unknown(String),
    InvalidArgumentCount { current: usize, expected: usize },
    InvalidArgument(&'static str),
}

impl fmt::Display for BadCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BadCommand::Unknown(cmd) => write!(f, "unknown command: {cmd}"),
            BadCommand::InvalidArgumentCount { current, expected } => write!(
                f,
                "invalid argument count: current={current}, expected={expected}"
            ),
            BadCommand::InvalidArgument(msg) => write!(f, "invalid argument: {msg}"),
        }
    }
}

impl Error for BadCommand {}

#[derive(Debug)]
pub enum ShellCommand {
    Quit,
    Help,
    Start,
    Stop,
    Status,
    Block,
    Poll,
    SendMessage(Message, SessionId),
    NoOperation,
}

impl FromStr for ShellCommand {
    type Err = BadCommand;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source.trim() {
            "quit" | "q" => Ok(Self::Quit),
            "help" | "?" => Ok(Self::Help),
            "start" => Ok(Self::Start),
            "stop" => Ok(Self::Stop),
            "status" => Ok(Self::Status),
            "block" => Ok(Self::Block),
            "poll" => Ok(Self::Poll),
            "" => Ok(Self::NoOperation),
            cmd if cmd.starts_with("send_to ") => {
                parse_send_to(cmd).map(|x| Self::SendMessage(x.0, x.1))
            }
            cmd => Err(BadCommand::Unknown(cmd.to_string())),
        }
    }
}

fn parse_send_to(source: &str) -> Result<(Message, SessionId), BadCommand> {
    // Split command into token
    let mut tokens = source.split_whitespace();
    debug_assert_eq!(tokens.next().as_deref(), Some("send_to"));

    let text_msg = tokens.next().ok_or(BadCommand::InvalidArgumentCount {
        current: 0,
        expected: 3,
    })?;
    let text_sender = tokens.next().ok_or(BadCommand::InvalidArgumentCount {
        current: 1,
        expected: 3,
    })?;
    let text_target = tokens.next().ok_or(BadCommand::InvalidArgumentCount {
        current: 2,
        expected: 3,
    })?;

    // Build message from 1st data token
    let mut msg = Message::new();
    for field in text_msg.split('|') {
        let mut field_tokens = field.splitn(2, '=');

        let tag = field_tokens
            .next()
            .ok_or(BadCommand::InvalidArgument("Invalid tag"))?
            .parse()
            .ok()
            .ok_or(BadCommand::InvalidArgument("Invalid tag number"))?;

        let value = field_tokens
            .next()
            .ok_or(BadCommand::InvalidArgument("Invalid value"))?;

        msg.set_field(tag, value)
            .ok()
            .expect("Fail to set msg tag=value");
    }

    // Build session ID from 2nd data token
    let session_id = SessionId::try_new("FIX.4.4", text_sender, text_target, "")
        .expect("Fail to allocate new session ID");

    Ok((msg, session_id))
}
