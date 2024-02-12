use quick_xml::events::BytesStart;

use crate::{read_attribute, FixSpecError, XmlObject, XmlReadable, XmlWritable, XmlWriter};

/// XML `<field><value ...>` representation.
#[derive(Debug, Clone)]
pub struct FieldAllowedValue {
    /// Associated value.
    pub value: String,
    /// Associated description / name.
    pub description: String,
}

impl XmlObject for FieldAllowedValue {
    const TAG_NAME: &'static str = "value";
}

impl XmlReadable for FieldAllowedValue {
    fn parse_xml_node(element: &BytesStart) -> Result<Self, FixSpecError> {
        let value = read_attribute(element, "enum")?;
        let description = read_attribute(element, "description")?;
        Ok(Self { value, description })
    }
}

impl XmlWritable for FieldAllowedValue {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> quick_xml::Result<&'a mut XmlWriter> {
        writer
            .create_element(Self::TAG_NAME)
            .with_attribute(("enum", self.value.as_str()))
            .with_attribute(("description", self.description.as_str()))
            .write_empty()
    }
}
