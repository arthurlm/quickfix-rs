use quick_xml::events::BytesStart;

use crate::{
    read_attribute, write_xml_list, FieldValue, FixSpecError, MessageCategory, XmlObject,
    XmlReadable, XmlReader, XmlWritable, XmlWriter,
};

/// XML `<message>` description.
#[derive(Debug, Clone)]
pub struct Message {
    /// Message name.
    pub name: String,
    /// `msgtype`.
    pub msg_type: String,
    /// Category level.
    pub category: MessageCategory,
    /// Possible sub-components.
    pub values: Vec<FieldValue>,
}

impl XmlObject for Message {
    const TAG_NAME: &'static str = "message";
}

impl XmlReadable for Message {
    fn parse_xml_node(element: &BytesStart) -> Result<Self, FixSpecError> {
        let name = read_attribute(element, "name")?;
        let msg_type = read_attribute(element, "msgtype")?;
        let category = MessageCategory::parse_xml_element(element)?;

        Ok(Self {
            name,
            msg_type,
            category,
            values: Vec::new(),
        })
    }

    fn parse_xml_tree(element: &BytesStart, reader: &mut XmlReader) -> Result<Self, FixSpecError> {
        let mut output = Self::parse_xml_node(element)?;
        output.values = FieldValue::parse_xml_tree(reader, Self::TAG_NAME)?;
        Ok(output)
    }
}

impl XmlWritable for Message {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> quick_xml::Result<&'a mut XmlWriter> {
        let element = writer
            .create_element(Self::TAG_NAME)
            .with_attribute(("name", self.name.as_str()))
            .with_attribute(("msgtype", self.msg_type.as_str()))
            .with_attribute(("msgcat", self.category.as_static_str()));

        if self.values.is_empty() {
            element.write_empty()
        } else {
            element.write_inner_content(|writer| write_xml_list(writer, &self.values))
        }
    }
}
