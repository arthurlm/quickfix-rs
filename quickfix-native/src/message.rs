use std::ops::{Deref, DerefMut};

use crate::{field_map::FieldMap, group::GroupOperations, message_order::MessageOrder};

/// Base class for all FIX messages.
/// A message consists of three field maps.
/// One for the header, the body, and the trailer.
#[derive(Debug, Clone)]
pub struct Message {
    header: Header,
    body: FieldMap,
    trailer: Trailer,
    valid_structure: bool,
    tag: i32,
}

impl GroupOperations for Message {
    fn get_field_map(&self) -> &FieldMap {
        &self.body
    }
    fn get_field_map_mut(&mut self) -> &mut FieldMap {
        &mut self.body
    }
}

//TODO:
impl Message {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            body: FieldMap::new_with_order(MessageOrder::normal()),
            trailer: Trailer::new(),
            valid_structure: true,
            tag: 0,
        }
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }
    pub fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    pub fn get_trailer(&self) -> &Trailer {
        &self.trailer
    }
    pub fn get_trailer_mut(&mut self) -> &mut Trailer {
        &mut self.trailer
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}
impl Deref for Message {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    field_map: FieldMap,
}
impl Header {
    pub fn new() -> Self {
        Self {
            field_map: FieldMap::new(),
        }
    }
}

impl GroupOperations for Header {
    fn get_field_map(&self) -> &FieldMap {
        &self.field_map
    }
    fn get_field_map_mut(&mut self) -> &mut FieldMap {
        &mut self.field_map
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Header {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.field_map
    }
}

impl DerefMut for Header {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field_map
    }
}

#[derive(Debug, Clone)]
pub struct Trailer {
    field_map: FieldMap,
}

impl Trailer {
    pub fn new() -> Self {
        Self {
            field_map: FieldMap::new(),
        }
    }
}
impl GroupOperations for Trailer {
    fn get_field_map(&self) -> &FieldMap {
        &self.field_map
    }
    fn get_field_map_mut(&mut self) -> &mut FieldMap {
        &mut self.field_map
    }
}
impl Default for Trailer {
    fn default() -> Self {
        Self::new()
    }
}
impl Deref for Trailer {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.field_map
    }
}

impl DerefMut for Trailer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field_map
    }
}
