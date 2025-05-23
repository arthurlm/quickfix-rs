use std::io;

use quick_xml::events::Event;

use crate::{
    Component, Field, FixSpecError, Group, XmlObject, XmlReadable, XmlReader, XmlWritable,
    XmlWriter,
};

/// Sub component possible value.
#[derive(Debug, Clone)]
pub enum FieldValue {
    /// Sub component is a field.
    Field(Field),
    /// Sub component is a group.
    Group(Group),
    /// Sub component is a factorized component.
    Component(Component),
}

impl FieldValue {
    pub(crate) fn parse_xml_tree(
        reader: &mut XmlReader,
        end_tag: &str,
    ) -> Result<Vec<Self>, FixSpecError> {
        let mut values = Vec::new();

        loop {
            match reader.read_event()? {
                Event::Empty(element) | Event::Start(element)
                    if element.name().as_ref() == Field::TAG_NAME.as_bytes() =>
                {
                    values.push(Self::Field(Field::parse_xml_tree(&element, reader)?));
                }
                Event::Empty(element) | Event::Start(element)
                    if element.name().as_ref() == Group::TAG_NAME.as_bytes() =>
                {
                    values.push(Self::Group(Group::parse_xml_tree(&element, reader)?));
                }
                Event::Empty(element) | Event::Start(element)
                    if element.name().as_ref() == Component::TAG_NAME.as_bytes() =>
                {
                    values.push(Self::Component(Component::parse_xml_tree(
                        &element, reader,
                    )?));
                }
                Event::End(element) if element.name().as_ref() == end_tag.as_bytes() => {
                    return Ok(values);
                }
                _ => {}
            }
        }
    }
}

impl XmlWritable for FieldValue {
    fn write_xml<'a>(&self, writer: &'a mut XmlWriter) -> io::Result<&'a mut XmlWriter> {
        match self {
            Self::Field(field) => field.write_xml(writer),
            Self::Group(group) => group.write_xml(writer),
            Self::Component(component) => component.write_xml(writer),
        }
    }
}
