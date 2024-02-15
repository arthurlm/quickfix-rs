use quick_xml::events::{BytesStart, Event};

use crate::{
    parse_xml_list, read_attribute, write_xml_container, ComponentSpec, FieldSpec, FieldValue,
    FixSpecError, XmlObject, XmlReadable, XmlReader, XmlWritable, XmlWriter,
};

use super::message::Message;

/// XML FIX dictionary description.
#[derive(Debug, Clone)]
pub struct FixSpec {
    /// FIX version number.
    pub version: (u8, u8, u8),
    /// Is FIXT ?
    pub is_fixt: bool,
    /// Message shared headers.
    pub headers: Vec<FieldValue>,
    /// Known FIX messages.
    pub messages: Vec<Message>,
    /// Message shared trailers.
    pub trailers: Vec<FieldValue>,
    /// Factorized components specs.
    pub component_specs: Vec<ComponentSpec>,
    /// Regular fields specs.
    pub field_specs: Vec<FieldSpec>,
}

impl FixSpec {
    /// Generate a new empty FIXT spec.
    pub fn new_fixt() -> Self {
        Self {
            version: (1, 1, 0),
            is_fixt: true,
            headers: vec![],
            messages: vec![],
            trailers: vec![],
            component_specs: vec![],
            field_specs: vec![],
        }
    }
}

impl XmlObject for FixSpec {
    const TAG_NAME: &'static str = "fix";
}

impl XmlReadable for FixSpec {
    fn parse_xml_node(element: &BytesStart) -> Result<Self, FixSpecError> {
        let version = (
            read_attribute(element, "major")?.parse()?,
            read_attribute(element, "minor")?.parse()?,
            read_attribute(element, "servicepack")?.parse()?,
        );

        let is_fixt = read_attribute(element, "type")? == "FIXT";

        Ok(Self {
            version,
            is_fixt,
            headers: Vec::new(),
            messages: Vec::new(),
            trailers: Vec::new(),
            component_specs: Vec::new(),
            field_specs: Vec::new(),
        })
    }

    fn parse_xml_tree(element: &BytesStart, reader: &mut XmlReader) -> Result<Self, FixSpecError> {
        let mut output = Self::parse_xml_node(element)?;

        loop {
            match reader.read_event()? {
                Event::Start(element) => match element.name().as_ref() {
                    b"header" => output.headers = FieldValue::parse_xml_tree(reader, "header")?,
                    b"messages" => output.messages = parse_xml_list(reader, "messages")?,
                    b"trailer" => output.trailers = FieldValue::parse_xml_tree(reader, "trailer")?,
                    b"components" => output.component_specs = parse_xml_list(reader, "components")?,
                    b"fields" => output.field_specs = parse_xml_list(reader, "fields")?,
                    _ => {}
                },
                Event::End(element) if element.name().as_ref() == Self::TAG_NAME.as_bytes() => {
                    break
                }
                _ => {}
            }
        }

        Ok(output)
    }
}

impl XmlWritable for FixSpec {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> quick_xml::Result<&'a mut XmlWriter> {
        writer
            .create_element(Self::TAG_NAME)
            .with_attribute(("type", if self.is_fixt { "FIXT" } else { "FIX" }))
            .with_attribute(("major", self.version.0.to_string().as_str()))
            .with_attribute(("minor", self.version.1.to_string().as_str()))
            .with_attribute(("servicepack", self.version.2.to_string().as_str()))
            .write_inner_content(|writer| {
                write_xml_container(writer, "header", &self.headers)?;
                write_xml_container(writer, "messages", &self.messages)?;
                write_xml_container(writer, "trailer", &self.trailers)?;
                write_xml_container(writer, "components", &self.component_specs)?;
                write_xml_container(writer, "fields", &self.field_specs)?;
                Ok(())
            })
    }
}
