use std::io;

use quick_xml::events::BytesStart;

use crate::{read_attribute, FixSpecError, XmlObject, XmlReadable, XmlWritable, XmlWriter};

/// XML `<field>` link.
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name.
    pub name: String,
    /// Field is required.
    pub required: bool,
}

impl XmlObject for Field {
    const TAG_NAME: &'static str = "field";
}

impl XmlReadable for Field {
    fn parse_xml_node(element: &BytesStart) -> Result<Self, FixSpecError> {
        let name = read_attribute(element, "name")?;
        let required = read_attribute(element, "required")? == "Y";
        Ok(Self { name, required })
    }
}

impl XmlWritable for Field {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> io::Result<&'a mut XmlWriter> {
        writer
            .create_element(Self::TAG_NAME)
            .with_attribute(("name", self.name.as_str()))
            .with_attribute(("required", if self.required { "Y" } else { "N" }))
            .write_empty()
    }
}
