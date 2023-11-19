# Quick fix binding for Rust

This project is an **unofficial** binding between [quickfix library](https://github.com/quickfix/quickfix) and rust project.

## What is it, what it is not ?

Main idea of this project is to provide a **minimal** safe and working bridge between rust and C++ quickfix library.

Not all methods, functions, classes, nor features of the original library will be exposed to Rust crate.
Target is just to have minimal features to create small and safe applications like:

- some program that can send order / receive messages to adjust strategy from.
- basic router with no smart order router capabilities.

What is already bind and working in this crate:

1. Basic structure binding `Settings` / `LogFactory` / `MessageStoreFactory` / `Message` / `Group` / `SessionID` / `...`.
2. Reading / writing messages.
3. Sending messages using session ID.
4. `SocketAcceptor` / `SocketInitiator`.
5. `Application` callbacks as a Rust trait.

What I do **not** plan to bind from this crate:

1. SSL support.

    Use binary like `stunnel` to emulate the feature.
    It is simpler to use than the original SSL quickfix, even if it add some performances overhead.

2. Python / Ruby binding.

    Use original library instead obviously.

3. Threaded versions of socket acceptor / initiator.

    Multithreading model is just too different between Rust / C++.
    It is much more simple to handle correctly multithreading from Rust side and use single thread C++ socket handler.

4. Autotools build toolchain.

    Just use `cmake` once and for all !

5. Struct to bind messages from XML spec.

    Most of the time, vendors / brokers have custom field that do not match auto-generated struct.
    To me they are not relevant most of the time.

    Moreover it is so simple to just create an Rust enum / struct that match your current model.
    Having all this messaging generated stuff just make the code more complicated for most of the need.

6. All binding of `LogFactory`.

    I just provide Rust standard trait.
    You can implement whatever you want using standard Rust crate and impl 3 callbacks (logger / redis / syslog / sql / ...).

    Moreover Rust file descriptor are protected by mutex, so this avoid mixing log from C++ / Rust in the same program.

7. SQL binding for `MessageStoreFactory`.

    For now I am not sure what the plan is ...
    I think target is same answer as `LogFactory` but it is not 100% clear for now.
    Things may change in the future.

8. Exotic operating system.

    AIX / Solaris are not targeted.
    They are not Rust [Tier1](https://doc.rust-lang.org/nightly/rustc/platform-support.html) for now.

## Is it ready for production ?

Yes. But keep in mind that not every feature of the original FIX library are available.
If some of your needs are missing: PR / feedbacks are welcomed 😁!

## How does it work ?

Since it is not possible (yet) to produce binding from Rust to C++ library, I have take another approach.

1. Create a C++ to C library: `quickfix-bind`.
2. Create a C to Rust unsafe library: `quickfix-ffi`.
3. Create a Rust unsafe to safe library: `quickfix`.

Check [DEV_NOTES](./doc/DEV_NOTES.md) for more information on the dev workflow and my research.

## How do I ?

Build C binding library:

```sh
mkdir target
cd target
CFLAGS="-I$HOME/.local/include" CXXFLAGS="-I$HOME/.local/include" LDFLAGS="-L$HOME/.local/lib" cmake \
    -DCMAKE_BUILD_TYPE=Release \
    -DQUICKFIX_BIND_EXAMPLES=ON \
    ..
make
```

Run C binding example:

```sh
LD_LIBRARY_PATH="$HOME/.local/lib" ./quickfix-bind/demo_basic_binding ../example/settings.ini
```

Rust FFI example:

```sh
cargo r --example demo_basic_ffi -- example/settings.ini
```

Run rust full binding example:

```sh
cargo r --example fix_getting_started -- example/server.ini
cargo r --example fix_repl -- acceptor example/server.ini
cargo r --example fix_repl -- initiator example/client.ini
```

## Build requirements

Following package must be install to build the library:

- `cmake`
- a C++ compiler (with C++17 support)
- `rustup` / `rustc` / `cargo` (obviously 😉)
