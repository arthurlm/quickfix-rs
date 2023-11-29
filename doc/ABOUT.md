# What is it, what it is not ?

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
    We are in 2023+ and not targeting OS from the 70s.

5. Struct to bind messages from XML spec.

    Most of the time, vendors / brokers have custom field that do not match auto-generated struct.
    To me they are not relevant most of the time.

    Moreover it is so simple to just create an Rust enum / struct that match your current model.
    Having all this messaging generated stuff just make the code more complicated for most of the need.

6. All binding of `LogFactory`.

    I just provide Rust standard trait.
    You can implement whatever you want using standard Rust crate and impl 3 callbacks (logger / redis / syslog / sql / ...).

    Moreover Rust file descriptor are protected by mutex, so this avoid mixing log from C++ / Rust in the same program.

7. Custom `MessageStoreFactory` from rust.

   For now, only `FileMessageStoreFactory` and `MemoryMessageStoreFactory` are bind.
   You can use also use `MySqlMessageStoreFactory` and `PostgresMessageStoreFactory` when enabling crate feature flag.
   Implementing message store from rust side is a little bit tricky and I am not 100% sure of the correct way to proceed.

8. Exotic operating system.

    AIX / Solaris are not targeted.
    They are not Rust [Tier1](https://doc.rust-lang.org/nightly/rustc/platform-support.html) for now.
