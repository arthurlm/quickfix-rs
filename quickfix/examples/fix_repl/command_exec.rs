use std::io::{self, stdin, stdout, BufRead, StdinLock, Write};

use quickfix::{send_to_target, ConnectionHandler};

use crate::command_parser::ShellCommand;

pub struct FixShell<'a> {
    stdin: StdinLock<'a>,
    last_command: String,
}

impl FixShell<'_> {
    pub fn new() -> Self {
        Self {
            stdin: stdin().lock(),
            last_command: String::with_capacity(1024),
        }
    }

    fn read_user_input(&mut self) -> io::Result<()> {
        let mut stdout = stdout().lock();
        write!(stdout, "FIX> ")?;
        stdout.flush()?;
        drop(stdout);

        self.last_command.clear();
        self.stdin.read_line(&mut self.last_command)?;

        Ok(())
    }

    fn exec_command<C: ConnectionHandler>(
        &mut self,
        command: ShellCommand,
        connection_handler: &mut C,
    ) {
        match command {
            ShellCommand::Help => {
                println!("Available commands:");
                println!("- status : Print connection handler status");
                println!("- start  : Start connection handler");
                println!("- block  : Block connection handler");
                println!("- poll   : Poll connection handler");
                println!("- stop   : Stop connection handler");
                println!("- send_to K1=V1|K2=V2|… sender target : Create new FIX message");
            }
            ShellCommand::Start => println!("RESULT: {:?}", connection_handler.start()),
            ShellCommand::Stop => println!("RESULT: {:?}", connection_handler.stop()),
            ShellCommand::Status => println!(
                "Connection handler status: logged_on={:?}, stopped={:?}",
                connection_handler.is_logged_on(),
                connection_handler.is_stopped(),
            ),
            ShellCommand::Block => println!("RESULT: {:?}", connection_handler.block()),
            ShellCommand::Poll => println!("RESULT: {:?}", connection_handler.poll()),
            ShellCommand::SendMessage(msg, session_id) => {
                println!("Sending {msg:?} to {session_id:?}");
                println!("SEND_RESULT: {:?}", send_to_target(msg, &session_id));
            }
            ShellCommand::NoOperation | ShellCommand::Quit => {}
        }
    }

    pub fn repl<C: ConnectionHandler>(&mut self, connection_handler: &mut C) {
        println!(">> Type 'help' or '?' for more information, 'quit' or 'q' to exit.");

        loop {
            self.read_user_input().expect("I/O error");

            // Handle CTRL-D
            if self.last_command.is_empty() {
                println!("CTRL-D");
                break;
            }
            // Handle other commands
            match self.last_command.parse::<ShellCommand>() {
                Ok(ShellCommand::Quit) => break,
                Ok(cmd) => self.exec_command(cmd, connection_handler),
                Err(err) => eprintln!("Error when running command: {err}"),
            }
        }
    }
}
