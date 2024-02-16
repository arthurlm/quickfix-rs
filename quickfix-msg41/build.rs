use std::{env, io};

use quickfix_msg_gen::*;

const SPEC_FILENAME: &str = "src/FIX41.xml";
const BEGIN_STRING: &str = "FIX.4.1";

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("Missing OUT_DIR");

    generate(SPEC_FILENAME, format!("{out_dir}/code.rs"), BEGIN_STRING)?;

    // Uncomment bellow line to show generated code
    // generate(SPEC_FILENAME, "src/out.rs", BEGIN_STRING)?;

    Ok(())
}
