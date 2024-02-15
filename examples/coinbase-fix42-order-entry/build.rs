use std::{env, io};

use quickfix_msg_gen::*;

const SPEC_FILENAME: &str = "src/cb-FIX42-prod-sand.xml";
const BEGIN_STRING: &str = "FIX.4.2";

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("Missing OUT_DIR");
    generate(SPEC_FILENAME, format!("{out_dir}/code.rs"), BEGIN_STRING)?;
    Ok(())
}
