use quick_xml::events::BytesStart;

use crate::{
    read_attribute, write_xml_list, FieldValue, FixSpecError, XmlObject, XmlReadable, XmlReader,
    XmlWritable, XmlWriter,
};

#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub required: bool,
    pub values: Vec<FieldValue>,
}

impl XmlObject for Group {
    const TAG_NAME: &'static str = "group";
}

impl XmlReadable for Group {
    fn parse_xml_node(element: &BytesStart) -> Result<Self, FixSpecError> {
        let name = read_attribute(element, "name")?;
        let required = read_attribute(element, "required")? == "Y";
        Ok(Self {
            name,
            required,
            values: Vec::new(),
        })
    }

    fn parse_xml_tree(element: &BytesStart, reader: &mut XmlReader) -> Result<Self, FixSpecError> {
        let mut output = Self::parse_xml_node(element)?;
        output.values = FieldValue::parse_xml_tree(reader, Self::TAG_NAME)?;
        Ok(output)
    }
}

impl XmlWritable for Group {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> quick_xml::Result<&'a mut XmlWriter> {
        writer
            .create_element(Self::TAG_NAME)
            .with_attribute(("name", self.name.as_str()))
            .with_attribute(("required", if self.required { "Y" } else { "N" }))
            .write_inner_content(|writer| write_xml_list(writer, &self.values))
    }
}
