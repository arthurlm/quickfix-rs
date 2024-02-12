use quick_xml::events::BytesStart;

use crate::{
    parse_xml_list, read_attribute, write_xml_list, FieldAllowedValue, FieldType, FixSpecError,
    XmlObject, XmlReadable, XmlReader, XmlWritable, XmlWriter,
};

/// XML `<field>` description.
#[derive(Debug, Clone)]
pub struct FieldSpec {
    /// FIX technical tag number.
    pub number: u32,
    /// FIX dictionary tag name.
    pub name: String,
    /// Value type.
    pub r#type: FieldType,
    /// Possible values in case there is a restricted list.
    pub values: Vec<FieldAllowedValue>,
}

impl XmlObject for FieldSpec {
    const TAG_NAME: &'static str = "field";
}

impl XmlReadable for FieldSpec {
    fn parse_xml_node(element: &BytesStart) -> Result<Self, FixSpecError> {
        let number = read_attribute(element, "number")?.parse()?;
        let name = read_attribute(element, "name")?;
        let r#type = read_attribute(element, "type")?.parse()?;

        Ok(Self {
            number,
            name,
            r#type,
            values: Vec::new(),
        })
    }

    fn parse_xml_tree(element: &BytesStart, reader: &mut XmlReader) -> Result<Self, FixSpecError> {
        let mut output = Self::parse_xml_node(element)?;
        output.values = parse_xml_list(reader, Self::TAG_NAME)?;
        Ok(output)
    }
}

impl XmlWritable for FieldSpec {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> quick_xml::Result<&'a mut XmlWriter> {
        let element = writer
            .create_element(Self::TAG_NAME)
            .with_attribute(("number", self.number.to_string().as_str()))
            .with_attribute(("name", self.name.as_str()))
            .with_attribute(("type", self.r#type.as_static_str()));

        if self.values.is_empty() {
            element.write_empty()
        } else {
            element.write_inner_content(|writer| write_xml_list(writer, &self.values))
        }
    }
}
