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

3. Autotools build toolchain.

    Just use `cmake` once and for all !
    We are in 2023+ and not targeting OS from the 80s.

4. FIX 5x messages generated code.

    FIX 5x XML definition is a little bit weird ...
    For example:
    - In [MatchType](https://www.onixs.biz/fix-dictionary/5.0/tagNum_574.html) some tag is defined multiple times.
      Generated enum are so inconsistent and cannot be safely generated.
    - There are probably other incompatibility but I stopped here ...

    You can edit XML spec to your need and create a package with desired spec locally.\
    Check FAQ for more info on this.

5. All binding of `LogFactory`.

    I just provide Rust standard trait.
    You can implement whatever you want using standard Rust crate and impl 3 callbacks (logger / redis / syslog / sql / ...).

    Moreover Rust file descriptor are protected by mutex, so this avoid mixing log from C++ / Rust in the same program.

6. Custom `MessageStoreFactory` from rust.

   For now, only `FileMessageStoreFactory` and `MemoryMessageStoreFactory` are bind.
   You can use also use `MySqlMessageStoreFactory` and `PostgresMessageStoreFactory` when enabling crate feature flag.
   Implementing message store from rust side is a little bit tricky and I am not 100% sure of the correct way to proceed.

7. Exotic operating system.

    AIX / Solaris are not targeted.
    They are not Rust [Tier1](https://doc.rust-lang.org/nightly/rustc/platform-support.html) for now.
