# Frequently Asked Questions

## What is the rust library overhead vs C++ quickfix

I have not benchmark it.

I have try to design rust wrapper with the minimal footprint as possible.
My guess is (if [LTO](https://doc.rust-lang.org/cargo/reference/profiles.html#lto) is enabled) overhead will be pretty low.

For example, if we look at `quickfix::Message`:

- it is a wrapper of `quickfix_ffi::FixMessage_t`
- which is a wrapper of `std::ptr::NonNull<ffi::c_void>`
- which will be used in `quickfix-bind` library to directly call quickfix c++ functions.

Moreover you can check `quickfix::Message` size is the size of a pointer.

## How can I use my own FIX XML spec file ?

You can take for example what I have done to generate [coinbase FIX 4.2](../examples/coinbase-fix42/) package:

1. Crate a new sub-package in your workspace (ex: `my-fix51`)
2. Add the FIX XML spec file in your `src` folder
3. Add `quickfix` to your dependency
4. Add `quickfix-msg-gen` to your **build** dependency
5. Add `src/lib.rs` with following content:

```rust
include!(concat!(env!("OUT_DIR"), "/code.rs"));
```

6. Add `build.rs` with following content:

```rust
use std::{env, io};

use quickfix_msg_gen::*;

const SPEC_FILENAME: &str = "src/my-spec-fix51.xml";
const BEGIN_STRING: &str = "FIX.5.1";

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("Missing OUT_DIR");
    generate(SPEC_FILENAME, format!("{out_dir}/code.rs"), BEGIN_STRING)?;
    Ok(())
}
```

## I am using FIX 5.0+ so I have multiple spec file, how can I build struct from it ?

Generating `struct` / `enum` from multiple XML spec file is hard.\
How do you track the source XML file ?\
How to deal with duplicated definition and message order ?

To resolve this issue it is simple: **you have to work with only one XML spec file**.\
I have few utilities in [`quickfix-spec-parser`](../quickfix-spec-parser/examples/) to help you merging multiple XML spec files.
It will not solve every problem, so you might still have to double check output XML on your own.

PR about improving this utilities are always welcomed 😁.

## Why coinbase example are not published ?

I do not own the "coinbase" name and do not own their XML spec file.\
I am also  not working for / with them.

So, I legally cannot publish theses examples.\
They are just there to show you how to make your own package from an XML spec file.

## How do I ?

Build C binding library:

```sh
mkdir build
cd build
CFLAGS="-I$HOME/.local/include" CXXFLAGS="-I$HOME/.local/include" LDFLAGS="-L$HOME/.local/lib" cmake \
    -DCMAKE_BUILD_TYPE=Release \
    -DQUICKFIX_BIND_EXAMPLES=ON \
    ../quickfix-ffi
make
```

Run C binding example:

```sh
LD_LIBRARY_PATH="$HOME/.local/lib" ./examples/demo_basic_binding ../examples/configs/settings.ini
```

Rust FFI example:

```sh
cargo r --example demo_basic_ffi -- examples/configs/settings.ini
```

Run rust full binding example:

```sh
cargo r --example fix_getting_started -- examples/configs/server.ini
cargo r --example fix_repl -- acceptor examples/configs/server.ini
cargo r --example fix_repl -- initiator examples/configs/client.ini
```
