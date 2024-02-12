use quick_xml::events::BytesStart;

use crate::{
    read_attribute, write_xml_list, FieldValue, FixSpecError, XmlObject, XmlReadable, XmlReader,
    XmlWritable, XmlWriter,
};

/// XML `<component>` description.
#[derive(Debug, Clone)]
pub struct ComponentSpec {
    /// Component name.
    pub name: String,
    /// Component content.
    pub values: Vec<FieldValue>,
}

impl XmlObject for ComponentSpec {
    const TAG_NAME: &'static str = "component";
}

impl XmlReadable for ComponentSpec {
    fn parse_xml_node(element: &BytesStart) -> Result<Self, FixSpecError> {
        let name = read_attribute(element, "name")?;
        Ok(Self {
            name,
            values: Vec::new(),
        })
    }

    fn parse_xml_tree(element: &BytesStart, reader: &mut XmlReader) -> Result<Self, FixSpecError> {
        let mut output = Self::parse_xml_node(element)?;
        output.values = FieldValue::parse_xml_tree(reader, Self::TAG_NAME)?;
        Ok(output)
    }
}

impl XmlWritable for ComponentSpec {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> quick_xml::Result<&'a mut XmlWriter> {
        let element = writer
            .create_element(Self::TAG_NAME)
            .with_attribute(("name", self.name.as_str()));

        if self.values.is_empty() {
            element.write_empty()
        } else {
            element.write_inner_content(|writer| write_xml_list(writer, &self.values))
        }
    }
}
