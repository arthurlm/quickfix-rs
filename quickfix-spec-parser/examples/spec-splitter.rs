use std::{collections::HashSet, env, error::Error, fs};

use quickfix_spec_parser::*;

fn main() -> Result<(), Box<dyn Error>> {
    let (Some(src_filename), Some(transport_filename), Some(dictionary_filename)) =
        (env::args().nth(1), env::args().nth(2), env::args().nth(3))
    else {
        panic!("Invalid program usage <src file> <transport file> <dictionary file>");
    };

    // Open and parse source spec.
    let src_data = fs::read(src_filename)?;
    let src_spec = parse_spec(&src_data)?;

    // Generate transport layer from spec.
    let mut transport = FixSpec::new_fixt();
    transport.headers = src_spec.headers.clone();
    transport.trailers = src_spec.trailers.clone();
    transport.messages = src_spec
        .messages
        .iter()
        .filter(|x| x.category == MessageCategory::Admin)
        .cloned()
        .collect();

    generate_spec_fields(&mut transport, &src_spec);

    // Generate dictionary messages from spec.
    let mut dictionary = FixSpec::new_fixt();
    dictionary.messages = src_spec
        .messages
        .iter()
        .filter(|x| x.category == MessageCategory::App)
        .cloned()
        .collect();
    dictionary
        .messages
        .sort_by_key(|x| (x.msg_type.len(), x.msg_type.clone()));

    generate_spec_fields(&mut dictionary, &src_spec);

    // Write generated spec.
    let transport_data = write_spec(&transport)?;
    fs::write(transport_filename, transport_data)?;

    let dictionary_data = write_spec(&dictionary)?;
    fs::write(dictionary_filename, dictionary_data)?;

    Ok(())
}

/// Generate components / field specs.
///
/// This method expect header / trailer / message fields are already filled.
fn generate_spec_fields(dst_spec: &mut FixSpec, src_spec: &FixSpec) {
    // Extend component list.
    let component_names = extract_spec_component_names(&src_spec, &dst_spec.messages);
    dst_spec.component_specs = src_spec
        .component_specs
        .iter()
        .filter(|x| component_names.contains(&x.name))
        .cloned()
        .collect();

    // Extend fields list.
    let field_names = extract_spec_fields_names(&dst_spec);
    dst_spec.field_specs = src_spec
        .field_specs
        .iter()
        .filter(|x| field_names.contains(&x.name))
        .cloned()
        .collect();
}

/// Recursively iterate over FIX spec to find all mentioned component.
fn extract_spec_component_names(spec: &FixSpec, messages: &[Message]) -> HashSet<String> {
    let mut output = HashSet::new();

    fn visit(spec: &FixSpec, values: &[FieldValue], acc: &mut HashSet<String>) {
        for value in values {
            match value {
                FieldValue::Field(_) => {}
                FieldValue::Group(sub_group) => {
                    visit(spec, &sub_group.values, acc);
                }
                FieldValue::Component(sub_component) => {
                    acc.insert(sub_component.name.clone());

                    // Recursively visit component from src spec.
                    let comp = spec
                        .component_specs
                        .iter()
                        .find(|x| x.name == sub_component.name)
                        .expect(&format!(
                            "Fail to find sub component in spec: {}",
                            sub_component.name
                        ));
                    visit(spec, &comp.values, acc);
                }
            }
        }
    }

    for message in messages {
        visit(spec, &message.values, &mut output);
    }

    output
}

/// Recursively iterate over FIX spec to find all mentioned fields.
fn extract_spec_fields_names(spec: &FixSpec) -> HashSet<String> {
    let mut output = HashSet::new();

    fn visit(values: &[FieldValue], acc: &mut HashSet<String>) {
        for value in values {
            match value {
                FieldValue::Field(sub_field) => {
                    acc.insert(sub_field.name.clone());
                }
                FieldValue::Group(sub_group) => {
                    acc.insert(sub_group.name.clone());
                    visit(&sub_group.values, acc);
                }
                FieldValue::Component(_) => {}
            }
        }
    }

    visit(&spec.headers, &mut output);
    visit(&spec.trailers, &mut output);

    for message in &spec.messages {
        visit(&message.values, &mut output);
    }

    for component in &spec.component_specs {
        visit(&component.values, &mut output);
    }

    output
}
