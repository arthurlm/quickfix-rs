/*! Code generator from XML FIX dictionary spec file. */

use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::{self, Stdio},
};

use crate::converter::convert_spec;
use crate::model::*;

use convert_case::{Case, Casing};

mod converter;
mod model;

use quickfix_spec_parser::{FieldSpec, FieldType};

trait FieldAccessorGenerator {
    fn getter_prefix_text(&self) -> &'static str;
    fn setter_prefix_text(&self) -> &'static str;
    fn caller_suffix_text(&self) -> &'static str;
}

/// Take a FIX XML spec file as `src` parameter and generated code to `dst` parameter.
pub fn generate<S: AsRef<Path>, D: AsRef<Path>>(
    src: S,
    dst: D,
    begin_string: &str,
) -> io::Result<()> {
    let spec_data = fs::read(src)?;
    let spec = quickfix_spec_parser::parse_spec(&spec_data).expect("Cannot parse FIX spec");
    let spec = convert_spec(spec);

    // Generate the code.
    println!("Generating code ...");
    let mut output = String::with_capacity(5 << 20); // 5Mo initial buffer
    generate_root(&mut output, begin_string);
    generate_field_ids(&mut output, &spec.field_specs);
    generate_field_types(&mut output, &spec.field_specs);
    generate_headers(&mut output, &spec.headers);
    generate_trailers(&mut output, &spec.trailers);
    generate_messages(&mut output, &spec.messages);
    generate_message_cracker(&mut output, &spec.messages);

    // Spawn a rustfmt daemon.
    let mut rustfmt = process::Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Send the code to rustfmt.
    println!("Formatting code ...");
    let mut rustfmt_in = rustfmt.stdin.take().expect("Fail to take rustfmt stdin");
    rustfmt_in.write_all(output.as_bytes())?;
    rustfmt_in.flush()?;
    drop(rustfmt_in); // Avoid infinite waiting !

    // Check output and write result.
    let rustfmt_out = rustfmt.wait_with_output()?;
    if !rustfmt_out.status.success() {
        println!("rustfmt stdout =======================");
        println!("{}", String::from_utf8_lossy(&rustfmt_out.stdout));
        println!("rustfmt stderr =======================");
        println!("{}", String::from_utf8_lossy(&rustfmt_out.stderr));

        panic!("Fail to run rustfmt");
    }

    // Write code to disk.
    println!("Writing code to disk ...");
    fs::write(dst, rustfmt_out.stdout)?;
    Ok(())
}

fn generate_root(output: &mut String, begin_string: &str) {
    output.push_str(&format!(
        r#" #[allow(unused_imports)]
            use quickfix::*;

            pub const FIX_BEGIN_STRING: &str = "{begin_string}";

            #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
            pub struct FixParseError;

            impl std::fmt::Display for FixParseError {{
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                    writeln!(f, "FIX parse error")
                }}
            }}

            impl std::error::Error for FixParseError {{}}

            pub struct GroupIterator<'a, T, I> {{
                parent: &'a T,
                clone_group_func: fn(&'a T, usize) -> Option<I>,
                current_index: usize,
            }}

            impl<T, I> Iterator for GroupIterator<'_, T, I> {{
                type Item = I;

                fn next(&mut self) -> Option<Self::Item> {{
                    self.current_index += 1;
                    (self.clone_group_func)(self.parent, self.current_index)
                }}
            }}

            "#
    ))
}

fn generate_field_ids(output: &mut String, field_specs: &[FieldSpec]) {
    output.push_str("pub mod field_id {\n");

    for field_spec in field_specs {
        output.push_str(&format!(
            "pub const {}: i32 = {};\n",
            field_spec.name.to_case(Case::Constant),
            field_spec.number
        ));
    }

    output.push_str("} // field_id\n\n");
}

fn generate_field_types(output: &mut String, field_specs: &[FieldSpec]) {
    output.push_str("pub mod field_types {\n");

    for field_spec in field_specs {
        if !field_spec.values.is_empty() {
            match &field_spec.r#type {
                FieldType::Int | FieldType::Long => {
                    generate_field_type_int_values(output, field_spec);
                }
                _ => {
                    generate_field_type_char_values(output, field_spec);
                }
            }
        } else {
            generate_field_type_alias(output, field_spec);
        }
    }

    output.push_str("} // field_types\n\n");
}

fn generate_field_type_int_values(output: &mut String, field_spec: &FieldSpec) {
    assert!(!field_spec.values.is_empty());
    assert!(matches!(
        field_spec.r#type,
        FieldType::Int | FieldType::Long
    ));

    let enum_name = field_spec.name.as_str();

    // Generate enum possible values.
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    output.push_str(&format!("pub enum {enum_name} {{\n"));
    for value in &field_spec.values {
        output.push_str(&format!(
            "{} = {},\n",
            value.description.to_case(Case::UpperCamel),
            value.value
        ));
    }
    output.push_str("}\n\n");

    generate_field_type_values(output, field_spec);
}

fn generate_field_type_char_values(output: &mut String, field_spec: &FieldSpec) {
    assert!(!field_spec.values.is_empty());

    let enum_name = field_spec.name.as_str();

    // Generate enum possible values.
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    output.push_str(&format!("pub enum {enum_name} {{\n"));
    for value in &field_spec.values {
        output.push_str(&format!(
            "{},\n",
            value.description.to_case(Case::UpperCamel)
        ));
    }
    output.push_str("}\n\n");

    generate_field_type_values(output, field_spec);
}

fn generate_field_type_values(output: &mut String, field_spec: &FieldSpec) {
    assert!(!field_spec.values.is_empty());

    let type_name = field_spec.name.as_str();

    // Generate method helpers.
    output.push_str(&format!(
        r#" impl {type_name} {{
                #[inline(always)]
                pub const fn from_const_bytes(s: &[u8]) -> Result<Self, crate::FixParseError> {{
                    match s {{
                    "#
    ));
    for value in &field_spec.values {
        output.push_str(&format!(
            "    b\"{}\" => Ok(Self::{}),\n",
            value.value,
            value.description.to_case(Case::UpperCamel),
        ));
    }
    output.push_str(
        r#"             _ => Err(crate::FixParseError),
                    }
                }
            }

            "#,
    );

    // Generate `FromStr`.
    output.push_str(&format!(
        r#" impl std::str::FromStr for {type_name} {{
                type Err = crate::FixParseError;
                fn from_str(s: &str) -> Result<Self, Self::Err> {{
                    Self::from_const_bytes(s.as_bytes())
                }}
            }}

            "#
    ));

    // Generate `IntoFixValue`.
    output.push_str(&format!(
        r#" impl quickfix::IntoFixValue for {type_name} {{
                fn into_fix_value(self) -> Result<std::ffi::CString, std::ffi::NulError> {{
                    std::ffi::CString::new(match self {{
                    "#
    ));
    for value in &field_spec.values {
        output.push_str(&format!(
            "    Self::{} => \"{}\",\n",
            value.description.to_case(Case::UpperCamel),
            value.value,
        ));
    }
    output.push_str(
        r#"         })
                }
            }

            "#,
    );
}

fn generate_field_type_alias(output: &mut String, field_spec: &FieldSpec) {
    assert!(field_spec.values.is_empty());

    let type_name = field_spec.name.as_str();
    let rust_type = match &field_spec.r#type {
        FieldType::Int => "i64",
        FieldType::Long => "i128",
        FieldType::Length => "u32",
        FieldType::SequenceNumber => "u32",
        FieldType::NumberInGroup => "i32",
        FieldType::Boolean => "bool",

        FieldType::Float
        | FieldType::Price
        | FieldType::Amount
        | FieldType::Quantity
        | FieldType::PriceOffset
        | FieldType::Percentage => "f64",

        FieldType::Char
        | FieldType::Data
        | FieldType::Time
        | FieldType::Date
        | FieldType::MonthYear
        | FieldType::DayOfMonth
        | FieldType::String
        | FieldType::Currency
        | FieldType::MultipleValueString
        | FieldType::Exchange
        | FieldType::LocalMarketDate
        | FieldType::UtcTimeStamp
        | FieldType::UtcDate
        | FieldType::UtcTimeOnly
        | FieldType::Country
        | FieldType::UtcDateOnly
        | FieldType::MultipleCharValue
        | FieldType::MultipleStringValue
        | FieldType::TzTimeOnly
        | FieldType::TzTimestamp
        | FieldType::XmlData
        | FieldType::Language
        | FieldType::TagNumber
        | FieldType::XidRef
        | FieldType::Xid
        | FieldType::LocalMarketTime => "String",
        x => unimplemented!("Unsupported FieldType: {x:?}"),
    };

    output.push_str(&format!("pub type {type_name} = {rust_type};\n\n"));
}

fn generate_headers(output: &mut String, components: &[SubComponent]) {
    struct Accessor;

    impl FieldAccessorGenerator for Accessor {
        fn getter_prefix_text(&self) -> &'static str {
            "inner.with_header(|x| x."
        }
        fn setter_prefix_text(&self) -> &'static str {
            "inner.with_header_mut(|x| x."
        }
        fn caller_suffix_text(&self) -> &'static str {
            ")"
        }
    }

    generate_message_wrapper(output, "Header", components, &Accessor);
}

fn generate_trailers(output: &mut String, components: &[SubComponent]) {
    struct Accessor;

    impl FieldAccessorGenerator for Accessor {
        fn getter_prefix_text(&self) -> &'static str {
            "inner.with_trailer(|x| x."
        }
        fn setter_prefix_text(&self) -> &'static str {
            "inner.with_trailer_mut(|x| x."
        }
        fn caller_suffix_text(&self) -> &'static str {
            ")"
        }
    }

    generate_message_wrapper(output, "Trailer", components, &Accessor);
}

fn generate_message_wrapper(
    output: &mut String,
    struct_name: &str,
    components: &[SubComponent],
    accessor: &impl FieldAccessorGenerator,
) {
    output.push_str(&format!(
        r#" #[derive(Debug)]
            pub struct {struct_name}<'a> {{ inner: &'a quickfix::Message }}

            "#
    ));

    generate_sub_components(output, &struct_name.to_case(Case::Snake), components);

    output.push_str(&format!("impl {struct_name}<'_> {{\n"));
    generate_components_getters(output, struct_name, components, accessor);
    output.push_str("}\n\n");

    output.push_str(&format!(
        r#" #[derive(Debug)]
            pub struct {struct_name}Mut<'a> {{ inner: &'a mut quickfix::Message }}

            "#
    ));

    output.push_str(&format!("impl {struct_name}Mut<'_> {{\n"));
    generate_components_setters(output, struct_name, components, accessor);
    output.push_str("}\n\n");
}

fn generate_messages(output: &mut String, messages: &[MessageSpec]) {
    for message in messages {
        generate_message(output, message);
    }
}

fn generate_message(output: &mut String, message: &MessageSpec) {
    let struct_name = message.name.as_str();
    let msg_type = message.msg_type.as_str();

    // Generate main struct content.
    output.push_str(&format!(
        r#" #[derive(Debug, Clone)]
            pub struct {struct_name} {{
                inner: quickfix::Message,
            }}

            impl {struct_name} {{
                pub const MSG_TYPE_BYTES: &'static str = "{msg_type}";
                pub const MSG_TYPE: crate::field_types::MsgType =
                    match crate::field_types::MsgType::from_const_bytes(Self::MSG_TYPE_BYTES.as_bytes()) {{
                        Ok(value) => value,
                        Err(_) => panic!("Invalid message type for {struct_name}"),
                    }};

                #[inline(always)]
                pub fn header(&mut self) -> Header {{
                    Header {{ inner: &self.inner }}
                }}

                #[inline(always)]
                pub fn header_mut(&mut self) -> HeaderMut {{
                    HeaderMut {{ inner: &mut self.inner }}
                }}

                #[inline(always)]
                pub fn trailer(&mut self) -> Trailer {{
                    Trailer {{ inner: &self.inner }}
                }}

                #[inline(always)]
                pub fn trailer_mut(&mut self) -> TrailerMut {{
                    TrailerMut {{ inner: &mut self.inner }}
                }}

                /// Convert inner message as FIX text.
                ///
                /// This method is only here for debug / tests purposes. Do not use this
                /// in real production code.
                #[inline(never)]
                pub fn to_fix_string(&self) -> String {{
                    self.inner
                        .to_fix_string()
                        .expect("Fail to format {struct_name} message as FIX string")
                }}
            }}

            impl From<{struct_name}> for quickfix::Message {{
                fn from(input: {struct_name}) -> Self {{
                    input.inner
                }}
            }}

            impl From<quickfix::Message> for {struct_name} {{
                fn from(input: quickfix::Message) -> Self {{
                    assert_eq!(
                        input
                            .with_header(|h| h.get_field(field_id::MSG_TYPE))
                            .and_then(|x| crate::field_types::MsgType::from_const_bytes(x.as_bytes()).ok()),
                        Some(Self::MSG_TYPE),
                    );
                    Self {{ inner: input }}
                }}
            }}

            "#
    ));

    // Generate default constructor
    let required_params = format_required_params(&message.components);
    let new_setters = format_new_setters(&message.components);

    output.push_str(&format!(
        r#" impl {struct_name} {{
                #[allow(clippy::too_many_arguments)]
                pub fn try_new({required_params}) -> Result<Self, quickfix::QuickFixError> {{
                    let mut inner = quickfix::Message::new();

                    // Set headers (most of them will be set by quickfix library).
                    inner.with_header_mut(|h| {{
                        h.set_field(crate::field_id::BEGIN_STRING, crate::FIX_BEGIN_STRING)
                    }})?;
                    inner.with_header_mut(|h| {{
                        h.set_field(crate::field_id::MSG_TYPE, Self::MSG_TYPE)
                    }})?;

                    // Set required attributes.
                    {new_setters}

                    Ok(Self {{ inner }})
                }}
            }}

            "#
    ));

    // Generate getter / setters and sub-components.
    struct Accessor;

    impl FieldAccessorGenerator for Accessor {
        fn getter_prefix_text(&self) -> &'static str {
            "inner."
        }
        fn setter_prefix_text(&self) -> &'static str {
            "inner."
        }
        fn caller_suffix_text(&self) -> &'static str {
            ""
        }
    }

    generate_sub_components(
        output,
        &message.name.to_case(Case::Snake),
        &message.components,
    );

    output.push_str(&format!("impl {struct_name} {{\n\n"));
    generate_components_getters(output, struct_name, &message.components, &Accessor);
    generate_components_setters(output, struct_name, &message.components, &Accessor);
    output.push_str("}\n\n");
}

fn generate_group(output: &mut String, group: &MessageGroup) {
    let struct_name = group.name.as_str();
    let group_id = format_field_id(&group.name);
    let group_delim = format_field_id(
        group
            .components
            .first()
            .expect("Group cannot be empty")
            .name(),
    );
    let group_value_ids = group
        .components
        .iter()
        .map(|x| format_field_id(x.name()))
        .collect::<Vec<_>>()
        .join(",");

    // Generate main struct.
    let required_params = format_required_params(&group.components);
    let new_setters = format_new_setters(&group.components);

    output.push_str(&format!(
        r#" #[derive(Debug, Clone)]
            pub struct {struct_name} {{
                pub(crate) inner: quickfix::Group,
            }}

            impl {struct_name} {{
                pub const FIELD_ID: i32 = {group_id};
                pub const DELIMITER: i32 = {group_delim};

                #[allow(clippy::too_many_arguments)]
                pub fn try_new({required_params}) -> Result<Self, quickfix::QuickFixError> {{
                    #[allow(unused_mut)]
                    let mut inner = quickfix::Group::try_with_orders(
                        Self::FIELD_ID,
                        Self::DELIMITER,
                        &[{group_value_ids}],
                    ).expect("Fail to build group {struct_name}");

                    {new_setters}

                    Ok(Self {{ inner }})
                }}
            }}

            "#
    ));

    // Generate getter / setters and sub-components.
    struct Accessor;

    impl FieldAccessorGenerator for Accessor {
        fn getter_prefix_text(&self) -> &'static str {
            "inner."
        }
        fn setter_prefix_text(&self) -> &'static str {
            "inner."
        }
        fn caller_suffix_text(&self) -> &'static str {
            ""
        }
    }

    generate_sub_components(output, &group.name.to_case(Case::Snake), &group.components);

    output.push_str(&format!("impl {struct_name} {{\n\n"));
    generate_components_getters(output, struct_name, &group.components, &Accessor);
    generate_components_setters(output, struct_name, &group.components, &Accessor);
    output.push_str("}\n\n");
}

fn generate_sub_components(output: &mut String, module_name: &str, components: &[SubComponent]) {
    // Check if message has some sub components
    if components
        .iter()
        .any(|x| matches!(x, SubComponent::Group(_)))
    {
        // Create dedicate module for the component.
        output.push_str(&format!(
            r#" pub mod {module_name} {{
                    use super::*;

                    "#
        ));

        for value in components {
            match value {
                SubComponent::Field(_) => {} // There is no sub-components to generate for a basic field
                SubComponent::Group(x) => {
                    generate_group(output, x);
                }
            }
        }
        output.push_str("}\n\n");
    }
}

fn generate_components_getters(
    output: &mut String,
    struct_name: &str,
    components: &[SubComponent],
    accessor: &impl FieldAccessorGenerator,
) {
    for component in components {
        match component {
            SubComponent::Field(x) => {
                generate_field_getter(output, &x.name, x.required, accessor);
            }
            SubComponent::Group(x) => {
                generate_group_reader(output, struct_name, x);
            }
        }
    }
}

fn generate_components_setters(
    output: &mut String,
    struct_name: &str,
    components: &[SubComponent],
    accessor: &impl FieldAccessorGenerator,
) {
    for component in components {
        match component {
            SubComponent::Field(x) => {
                generate_field_setters(output, x, accessor);
            }
            SubComponent::Group(x) => {
                generate_fn_add_group(output, struct_name, x);
            }
        }
    }
}

fn generate_field_getter(
    output: &mut String,
    field_name: &str,
    field_required: bool,
    accessor: &impl FieldAccessorGenerator,
) {
    // Eval trait and make some string alias.
    let call_get_prefix = accessor.getter_prefix_text();
    let call_suffix = accessor.caller_suffix_text();

    let fun_name = format!("get_{}", field_name.to_case(Case::Snake));
    let field_type = format!("crate::field_types::{field_name}");
    let field_id = format_field_id(field_name);

    // Generate code.
    if field_required {
        // Generate a getter that `panic()` if field is not set.
        output.push_str(&format!(
            r#" #[inline(always)]
                pub fn {fun_name}(&self) -> {field_type} {{
                    self.{call_get_prefix}get_field({field_id}){call_suffix}
                       .and_then(|x| x.parse().ok())
                       .expect("{field_id} is required but it is missing")
                }}

                "#
        ));
    } else {
        // Generate an optional getter.
        output.push_str(&format!(
            r#" #[inline(always)]
                pub fn {fun_name}(&self) -> Option<{field_type}> {{
                    self.{call_get_prefix}get_field({field_id}){call_suffix}
                       .and_then(|x| x.parse().ok())
                }}

                "#
        ));
    }
}

fn generate_field_setters(
    output: &mut String,
    field: &MessageField,
    accessor: &impl FieldAccessorGenerator,
) {
    // Eval trait and make some string alias.
    let call_set_prefix = accessor.setter_prefix_text();
    let call_suffix = accessor.caller_suffix_text();

    let field_name = field.name.to_case(Case::Snake);
    let field_type = format!("crate::field_types::{}", field.name);
    let field_id = format_field_id(&field.name);

    // Generate code.
    output.push_str(&format!(
        r#" #[inline(always)]
            pub fn set_{field_name}(&mut self, value: {field_type}) -> Result<&Self, quickfix::QuickFixError> {{
                self.{call_set_prefix}set_field({field_id}, value){call_suffix}?;
                Ok(self)
            }}

            "#
    ));

    // If field is optional, we can generate a remover.
    if !field.required {
        output.push_str(&format!(
            r#" #[inline(always)]
                pub fn remove_{field_name}(&mut self) -> Result<&Self, quickfix::QuickFixError> {{
                    self.{call_set_prefix}remove_field({field_id}){call_suffix}?;
                    Ok(self)
                }}

                "#
        ));
    }
}

fn generate_group_reader(output: &mut String, struct_name: &str, group: &MessageGroup) {
    // Add some type alias.
    let group_name = group.name.to_case(Case::Snake);
    let group_type = format!("self::{}::{}", struct_name.to_case(Case::Snake), group.name);

    // Generate code.
    output.push_str(&format!(
        r#" #[inline(always)]
            pub fn {group_name}_len(&self) -> usize {{
                self.inner
                    .get_field({group_type}::FIELD_ID)
                    .and_then(|x| x.parse().ok())
                    .unwrap_or_default()
            }}

            #[inline(always)]
            pub fn clone_group_{group_name}(&self, index: usize) -> Option<{group_type}> {{
                self.inner
                    .clone_group(index as i32, {group_type}::FIELD_ID)
                    .map(|raw_group| {group_type} {{ inner: raw_group }})
            }}

            #[inline(always)]
            pub fn iter_{group_name}(&self) -> GroupIterator<'_, Self, {group_type}> {{
                GroupIterator {{
                    parent: self,
                    clone_group_func: |parent, idx| parent.clone_group_{group_name}(idx),
                    current_index: 0,
                }}
            }}

            "#
    ));
}

fn generate_fn_add_group(output: &mut String, struct_name: &str, group: &MessageGroup) {
    // Add some type alias.
    let group_name = group.name.to_case(Case::Snake);
    let group_type = format!("self::{}::{}", struct_name.to_case(Case::Snake), group.name);

    // Generate code.
    output.push_str(&format!(
        r#" #[inline(always)]
            pub fn add_{group_name}(&mut self, value: {group_type}) -> Result<&Self, quickfix::QuickFixError> {{
                self.inner.add_group(&value.inner)?;
                Ok(self)
            }}

            "#
    ));
}

fn generate_message_cracker(output: &mut String, messages: &[MessageSpec]) {
    // Generate enum with all possible messages.
    output.push_str(
        r#" #[derive(Debug, Clone)]
            pub enum Messages {
            "#,
    );
    for message in messages {
        let struct_name = &message.name;

        output.push_str(&format!("  {struct_name}({struct_name}),\n"));
    }
    output.push_str(
        r#" }
            "#,
    );

    // Generate decode helpers.
    output.push_str(
        r#" impl Messages {
                /// Try decoding input message or return the message if it does not match any known message type.
                pub fn decode(input: quickfix::Message) -> Result<Self, quickfix::Message> {
                    match input
                        .with_header(|h| h.get_field(crate::field_id::MSG_TYPE))
                        .as_deref()
                    {
            "#,
    );
    for message in messages {
        let struct_name = &message.name;
        let message_type = &message.msg_type;

        output.push_str(&format!(
            "  Some(\"{message_type}\") => Ok(Self::{struct_name}(input.into())),\n"
        ));
    }
    output.push_str(
        r#"             _ => Err(input),
                    }
                }
            }
            "#,
    );
}

fn format_field_id(input: &str) -> String {
    format!("crate::field_id::{}", input.to_case(Case::Constant))
}

fn format_required_params(components: &[SubComponent]) -> String {
    components
        .iter()
        .filter(|x| x.is_required())
        .map(|x| {
            let name = x.name();
            let param_name = name.to_case(Case::Snake);
            format!("{param_name}: crate::field_types::{name}")
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_new_setters(components: &[SubComponent]) -> String {
    components
        .iter()
        .filter(|x| x.is_required())
        .map(|x| {
            let name = x.name();
            let field_id = format_field_id(name);
            let param_name = name.to_case(Case::Snake);
            format!("inner.set_field({field_id}, {param_name})?;")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
