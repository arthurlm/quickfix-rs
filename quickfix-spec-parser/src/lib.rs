#![warn(missing_docs)]

/*! FIX XML file spec parser.
 *
 * Allow reading official FIX XML dictionary and convert it to a struct / enum model.
 *
 * **NOTE** This crate is not a code generator. It only help having a clear representation of what FIX dictionary are.
 */

use std::io;

use bytes::Bytes;
use quick_xml::{events::Event, Reader, Writer};

mod error;
mod model {
    mod component;
    mod component_spec;
    mod field;
    mod field_allowed_value;
    mod field_spec;
    mod field_type;
    mod field_value;
    mod group;
    mod message;
    mod message_category;
    mod spec;

    pub use self::component::Component;
    pub use self::component_spec::ComponentSpec;
    pub use self::field::Field;
    pub use self::field_allowed_value::FieldAllowedValue;
    pub use self::field_spec::FieldSpec;
    pub use self::field_type::FieldType;
    pub use self::field_value::FieldValue;
    pub use self::group::Group;
    pub use self::message::Message;
    pub use self::message_category::MessageCategory;
    pub use self::spec::FixSpec;
}
mod xml_ext;

pub use error::*;
pub use model::*;
use xml_ext::*;

#[doc(hidden)] // For testing
pub use xml_ext::read_attribute;

type XmlWriter = Writer<Vec<u8>>;
type XmlReader<'a> = Reader<&'a [u8]>;

/// Try converting byte array into a FIX spec tree.
pub fn parse_spec(input: &[u8]) -> Result<FixSpec, FixSpecError> {
    let mut reader = Reader::from_reader(input);
    reader.config_mut().trim_text(true);

    match reader.read_event()? {
        // If we are at start of FIX spec.
        Event::Start(e) if e.name().as_ref() == FixSpec::TAG_NAME.as_bytes() => {
            FixSpec::parse_xml_tree(&e, &mut reader)
        }
        // Otherwise document is invalid
        _ => Err(FixSpecError::InvalidDocument("invalid root")),
    }
}

/// Convert FIX spec tree into a byte array.
pub fn write_spec(spec: &FixSpec) -> io::Result<Bytes> {
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 1);

    spec.write_xml(&mut writer)?;

    Ok(Bytes::from(writer.into_inner()))
}
