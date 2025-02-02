use std::io;

use quick_xml::events::{BytesStart, Event};

use crate::{FixSpecError, XmlReader, XmlWriter};

#[doc(hidden)]
pub fn read_attribute(item: &BytesStart, name: &str) -> Result<String, FixSpecError> {
    let attr = item
        .attributes()
        .filter_map(|x| x.ok())
        .find(|x| x.key.as_ref() == name.as_bytes())
        .ok_or_else(|| FixSpecError::InvalidAttribute(name.to_string()))?;

    let value = String::from_utf8(attr.value.to_vec())?;

    Ok(value)
}

pub trait XmlObject {
    const TAG_NAME: &'static str;
}

pub trait XmlWritable {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> io::Result<&'a mut XmlWriter>;
}

pub fn write_xml_list<T: XmlWritable>(writer: &mut XmlWriter, items: &[T]) -> io::Result<()> {
    for item in items {
        item.write_xml(writer)?;
    }
    Ok(())
}

pub fn write_xml_container<'a, T: XmlWritable>(
    writer: &'a mut XmlWriter,
    tag_name: &'a str,
    items: &[T],
) -> io::Result<&'a mut XmlWriter> {
    let element = writer.create_element(tag_name);

    if items.is_empty() {
        element.write_empty()
    } else {
        element.write_inner_content(|writer| write_xml_list(writer, items))
    }
}

pub trait XmlReadable: XmlObject {
    fn parse_xml_node(element: &BytesStart) -> Result<Self, FixSpecError>
    where
        Self: Sized;

    #[allow(unused_variables)]
    fn parse_xml_tree(element: &BytesStart, reader: &mut XmlReader) -> Result<Self, FixSpecError>
    where
        Self: Sized,
    {
        Self::parse_xml_node(element)
    }
}

pub fn parse_xml_list<T: XmlReadable>(
    reader: &mut XmlReader,
    end_tag: &str,
) -> Result<Vec<T>, FixSpecError> {
    let mut output = Vec::new();

    loop {
        match reader.read_event()? {
            Event::Start(element) if element.name().as_ref() == T::TAG_NAME.as_bytes() => {
                output.push(T::parse_xml_tree(&element, reader)?);
            }
            Event::Empty(element) if element.name().as_ref() == T::TAG_NAME.as_bytes() => {
                output.push(T::parse_xml_node(&element)?);
            }
            Event::End(element) if element.name().as_ref() == end_tag.as_bytes() => {
                return Ok(output);
            }
            _ => {}
        }
    }
}
