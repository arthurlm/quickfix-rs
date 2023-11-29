# Quick fix binding for Rust

![CI workflow](https://github.com/arthurlm/quickfix-rs/actions/workflows/ci.yml/badge.svg)

This project is an **unofficial** binding between [quickfix library](https://github.com/quickfix/quickfix) and rust project.

## Features

- Provide basic and safe API wrapper above [quickfix](https://github.com/quickfix/quickfix) library.
- Run on any hardware and operating system supported by Rust Tier 1.
- Message decoding / encoding including run-time validation.
- Session state storage options: SQL, File, In Memory.
- Logging options: stdout, stderr, [log](https://crates.io/crates/log) or any other crate if you implement your own trait.

## Documentation

- [What is it, what it is not ?](./doc/ABOUT.md)
- [FAQ](./doc/FAQ.md)
- [Internal design](./doc/DEV_NOTES.md)

## Examples

Here is the minimal application you can write to getting started with quickfix:

```rust
use std::{
    env,
    io::{stdin, Read},
    process::exit,
};

use quickfix::*;

#[derive(Default)]
pub struct MyApplication;

impl ApplicationCallback for MyApplication {
    // Implement whatever callback you need

    fn on_create(&self, _session: &SessionId) {
        // Do whatever you want here 😁
    }
}

fn main() -> Result<(), QuickFixError> {
    let args: Vec<_> = env::args().collect();
    let Some(config_file) = args.get(1) else {
        eprintln!("Bad program usage: {} <config_file>", args[0]);
        exit(1);
    };

    let settings = SessionSettings::try_from_path(config_file)?;
    let store_factory = FileMessageStoreFactory::try_new(&settings)?;
    let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;
    let app = Application::try_new(&MyApplication)?;

    let mut acceptor = SocketAcceptor::try_new(&settings, &app, &store_factory, &log_factory)?;
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
```

You may consider checking out this [directory](./quickfix/examples/) for more examples.

## Is it ready for production ?

Yes. But keep in mind that not every feature of the original FIX library are available.
If some of your needs are missing: PR / feedbacks are welcomed 😁!

## Build requirements

Following package must be install to build the library:

- `cmake`
- a C++ compiler (with C++17 support)
- `rustup` / `rustc` / `cargo` (obviously 😉)
