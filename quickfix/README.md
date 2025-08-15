# QuickFIX Rust

![CI workflow](https://github.com/arthurlm/quickfix-rs/actions/workflows/ci.yml/badge.svg)
![MSRV](https://img.shields.io/badge/MSRV-1.70.0-blue)
[![codecov](https://codecov.io/gh/arthurlm/quickfix-rs/graph/badge.svg?token=WVEWW996GO)](https://codecov.io/gh/arthurlm/quickfix-rs)
[![dependency status](https://deps.rs/repo/github/arthurlm/quickfix-rs/status.svg)](https://deps.rs/repo/github/arthurlm/quickfix-rs)

This project is an **unofficial** binding between [quickfix](https://github.com/quickfix/quickfix) library and Rust projects.

## Features

- Provide basic and safe API wrapper above [quickfix](https://github.com/quickfix/quickfix) library.
- Run on any hardware and operating system supported by Rust Tier 1 (Windows 7+, MacOS 10.12+ & Linux).
- Only include and compile what you need since project is split into minimal crates.
- Message decoding / encoding including run-time validation.
- Supports FIX versions 4x (version 5x can be build locally from XML spec file).
- Spec driven run-time message validation.
- Spec driven code generation of type-safe FIX messages, fields, and repeating groups.
- Session state storage options: SQL, File, In Memory.
- Logging options: stdout, stderr, [log](https://crates.io/crates/log) or any other crate if you implement your own trait.

## Documentation

- [What is it, what it is not ?](https://github.com/arthurlm/quickfix-rs/blob/main/doc/ABOUT.md)
- [FAQ](https://github.com/arthurlm/quickfix-rs/blob/main/doc/FAQ.md)
- [Internal design](https://github.com/arthurlm/quickfix-rs/blob/main/doc/DEV_NOTES.md)
- [Examples](https://github.com/arthurlm/quickfix-rs-examples)

External website:

- crates.io:
  [QuickFix](https://crates.io/crates/quickfix),
  [QuickFix FFI](https://crates.io/crates/quickfix-ffi)
  [QuickFix spec parser](https://crates.io/crates/quickfix-spec-parser)
  [QuickFix message generator](https://crates.io/crates/quickfix-msg-gen)
  [FIX 4.0](https://crates.io/crates/quickfix-msg40)
  [FIX 4.1](https://crates.io/crates/quickfix-msg41)
  [FIX 4.2](https://crates.io/crates/quickfix-msg42)
  [FIX 4.3](https://crates.io/crates/quickfix-msg43)
  [FIX 4.4](https://crates.io/crates/quickfix-msg44)
  [FIX 5.0](https://crates.io/crates/quickfix-msg50)
- docs.rs:
  [QuickFix](https://docs.rs/quickfix/latest/quickfix/)
  [QuickFix FFI](https://docs.rs/quickfix-ffi/latest/quickfix_ffi/)
  [QuickFix spec parser](https://docs.rs/quickfix-spec-parser/latest/quickfix_spec_parser/)
  [QuickFix message generator](https://docs.rs/quickfix-msg-gen/latest/quickfix_msg_gen/)
  [FIX 4.0](https://docs.rs/quickfix-msg40/latest/quickfix_msg40/)
  [FIX 4.1](https://docs.rs/quickfix-msg41/latest/quickfix_msg41/)
  [FIX 4.2](https://docs.rs/quickfix-msg42/latest/quickfix_msg42/)
  [FIX 4.3](https://docs.rs/quickfix-msg43/latest/quickfix_msg43/)
  [FIX 4.4](https://docs.rs/quickfix-msg44/latest/quickfix_msg44/)
  [FIX 5.0](https://docs.rs/quickfix-msg50/latest/quickfix_msg50/)

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
```

You may consider checking out this [repository](https://github.com/arthurlm/quickfix-rs-examples) for more examples.

## Is it ready for production ?

Yes. But keep in mind that not every feature of the original FIX library are available.
If some of your needs are missing: PR / feedbacks are welcomed 😁!

**API MAY CHANGE IN FUTURE VERSION**\
Crate is still in the [reviewing process](https://github.com/quickfix/quickfix/issues/533).
Feel free to participate and share your point of view on this github issue.

For list of breaking changes between version, please check [CHANGELOG](./CHANGELOG.md).

**NOTE**: I am personally not using for now the generated message struct.
I know they works fine thanks to unit tests and can be used in production code.
Feedback on this part are welcomed !

## Project status

I am not actively developing many more features to this project.\
To me it is actually pretty completed !

If something is missing to you, feel free to open an issue / create PR. Contribution are welcomed.

## Build requirements

Following package must be install to build the library:

- `cmake`
- a C++ compiler (with C++17 support)
- `rustup` / `rustc` / `cargo` (obviously 😉)
- `rustfmt` for auto generated messages from spec.
