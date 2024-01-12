use std::{env::temp_dir, fs, io};

use quickfix_msg_gen::generate;

#[test]
fn test_no_crash() -> io::Result<()> {
    let dump_path = temp_dir().join("test_quickfix_msg_gen");
    fs::create_dir_all(&dump_path)?;

    // Just test generator do not crash with standard FIX spec files.
    //
    // 🚨🚨 Output code is not compiled 🚨🚨 !
    // This will be done via sub-packages with a nicer output to debug what goes wrong.
    generate(
        "../quickfix-ffi/libquickfix/spec/FIX40.xml",
        dump_path.join("out40.rs"),
        "FIX.4.0",
    )?;
    generate(
        "../quickfix-ffi/libquickfix/spec/FIX41.xml",
        dump_path.join("out41.rs"),
        "FIX.4.1",
    )?;
    generate(
        "../quickfix-ffi/libquickfix/spec/FIX42.xml",
        dump_path.join("out42.rs"),
        "FIX.4.2",
    )?;
    generate(
        "../quickfix-ffi/libquickfix/spec/FIX43.xml",
        dump_path.join("out43.rs"),
        "FIX.4.3",
    )?;
    generate(
        "../quickfix-ffi/libquickfix/spec/FIX44.xml",
        dump_path.join("out44.rs"),
        "FIX.4.4",
    )?;

    Ok(())
}
