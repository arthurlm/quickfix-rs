use quick_xml::events::BytesStart;

use crate::{read_attribute, FixSpecError, XmlObject, XmlReadable, XmlWritable, XmlWriter};

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub required: bool,
}

impl XmlObject for Component {
    const TAG_NAME: &'static str = "component";
}

impl XmlReadable for Component {
    fn parse_xml_node(element: &BytesStart) -> Result<Self, FixSpecError> {
        let name = read_attribute(element, "name")?;
        let required = read_attribute(element, "required")? == "Y";
        Ok(Self { name, required })
    }
}

impl XmlWritable for Component {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> quick_xml::Result<&'a mut XmlWriter> {
        writer
            .create_element(Self::TAG_NAME)
            .with_attribute(("name", self.name.as_str()))
            .with_attribute(("required", if self.required { "Y" } else { "N" }))
            .write_empty()
    }
}
