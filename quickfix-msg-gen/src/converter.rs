use std::collections::HashSet;

use quickfix_spec_parser::{FieldValue, FixSpec, Message};

use crate::{FixCodeSpec, MessageField, MessageGroup, MessageSpec, SubComponent};

pub fn convert_spec(src: &FixSpec) -> FixCodeSpec {
    FixCodeSpec {
        field_specs: src.field_specs.clone(),
        headers: convert_field_value_list(src, &src.headers),
        trailers: convert_field_value_list(src, &src.trailers),
        messages: convert_messages(src, &src.messages),
    }
}

fn convert_field_value_list(spec: &FixSpec, values: &[FieldValue]) -> Vec<SubComponent> {
    let mut output = Vec::with_capacity(values.len());

    // Convert recursively `FieldValue` to `SubComponent`.
    for value in values {
        match value {
            FieldValue::Field(x) => output.push(SubComponent::Field(MessageField {
                name: x.name.clone(),
                required: x.required,
            })),
            FieldValue::Group(x) => output.push(SubComponent::Group(MessageGroup {
                name: x.name.clone(),
                components: convert_field_value_list(spec, &x.values),
            })),
            FieldValue::Component(component) => {
                let component_spec = spec
                    .component_specs
                    .iter()
                    .find(|x| x.name == component.name)
                    .expect("Cannot find component");

                output.extend(convert_field_value_list(spec, &component_spec.values));
            }
        }
    }

    // Remove duplicate: example FIX4.3 -> RegistrationInstructions -> OwnershipType.
    let mut uniques = HashSet::new();
    output.retain(|x| uniques.insert(x.name().to_string()));

    output
}

fn convert_messages(spec: &FixSpec, messages: &[Message]) -> Vec<MessageSpec> {
    messages
        .iter()
        .map(|message| MessageSpec {
            name: message.name.clone(),
            msg_type: message.msg_type.clone(),
            components: convert_field_value_list(spec, &message.values),
        })
        .collect()
}
