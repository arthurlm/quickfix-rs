use std::{env, error::Error, fs};

use quickfix_spec_parser::*;

fn main() -> Result<(), Box<dyn Error>> {
    let (Some(src_filename), Some(dst_filename)) = (env::args().nth(1), env::args().nth(2)) else {
        panic!("Invalid program usage <src file> <dst file>");
    };

    // Parse input file/
    let src_data = fs::read(src_filename)?;
    let src = parse_spec(&src_data)?;

    // Create new spec from src and reorder fields.
    let mut dst = src;
    dst.messages
        .sort_by_key(|x| (x.msg_type.len(), x.msg_type.clone()));

    for field_spec in &mut dst.field_specs {
        field_spec
            .values
            .sort_by_key(|x| (x.value.len(), x.value.clone()));
    }

    // Write output file.
    let dst_data = write_spec(&dst)?;
    fs::write(dst_filename, dst_data)?;

    Ok(())
}
