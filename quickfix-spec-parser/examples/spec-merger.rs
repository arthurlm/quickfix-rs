use std::{collections::HashMap, env::args, fs};

use quickfix_spec_parser::*;

fn main() {
    let spec_merged = args()
        .skip(1)
        .map(|filename| {
            eprintln!("Parsing file: {filename}");
            let file_content = fs::read(filename).expect("Cannot read file");
            let spec = parse_spec(&file_content).expect("Cannot parse spec");
            spec
        })
        .fold(FixSpec::new_fixt(), |mut acc, item| {
            // Extend un-sortable fields
            acc.headers.extend(item.headers);
            acc.trailers.extend(item.trailers);
            acc.component_specs.extend(item.component_specs);

            // Extends messages and sort them by msgtype
            acc.messages.extend(item.messages);
            acc.messages.sort_by_key(|x| x.msg_type.clone());

            // Merge fields
            acc.field_specs = merge_field_specs(&acc.field_specs, &item.field_specs);

            acc
        });

    let output_xml = write_spec(&spec_merged).expect("Fail to write spec");
    let txt = String::from_utf8(output_xml)
        .expect("generated XML is not UTF8")
        .replace('\"', "'");

    println!("{txt}");
}

fn merge_field_specs(a: &[FieldSpec], b: &[FieldSpec]) -> Vec<FieldSpec> {
    // Create a new map to merge all fields using their FIX number.
    let mut all_fields: HashMap<u32, FieldSpec> = HashMap::with_capacity(a.len() + b.len());

    for field_spec in a.iter().chain(b) {
        let entry = all_fields
            .entry(field_spec.number)
            .or_insert_with(|| FieldSpec {
                number: field_spec.number,
                name: field_spec.name.clone(),
                r#type: field_spec.r#type.clone(),
                values: Vec::new(),
            });

        entry.values.extend(field_spec.values.clone());
        entry
            .values
            .sort_by_key(|x| (x.value.len(), x.value.clone()));
        entry.values.dedup_by_key(|x| x.value.clone());
    }

    // Build output vec.
    let mut output: Vec<_> = all_fields.into_values().collect();
    output.sort_by_key(|x| x.number);
    output
}
