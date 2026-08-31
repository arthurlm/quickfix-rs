use crate::{
    errors::{NativeError, Result},
    field_map::FieldMap,
    message_order::MessageOrder,
};

#[derive(Debug, Clone)]
pub struct Group {
    fieldmap: FieldMap,
    /// group's identifying field tag9
    field_id: u32,
    /// delimiter field that  starts each group entry
    delim: u32,
}

pub trait GroupOperations {
    fn get_field_map(&self) -> &FieldMap;
    fn get_field_map_mut(&mut self) -> &mut FieldMap;

    /// add a group
    fn add_group(&mut self, group: &Group) {
        self.get_field_map_mut().add_group(group.clone());
    }

    /// replace a group at a specific index
    fn replace_group(&mut self, num: usize, group: &Group) -> Result<()> {
        self.get_field_map_mut()
            .replace_group(group.field(), num, group.clone())
    }

    /// get a group at a specific group
    fn get_group(&self, num: usize, group: &Group) -> Result<Group> {
        self.get_field_map()
            .get_group(group.field(), num)
            .ok_or_else(|| {
                crate::errors::NativeError::FieldNotFound(format!(
                    "Group not found at index {} for field {}",
                    num,
                    group.field()
                ))
            })
            .cloned()
    }
    /// remove a group at a specific index
    fn remove_group(&mut self, num: usize, group: &Group) -> Result<Group> {
        self.get_field_map_mut().remove_group(group.field(), num)
    }

    /// remove all groups of this type
    fn remove_group_by_field(&mut self, group: &Group) -> Result<Vec<Group>> {
        self.get_field_map_mut()
            .remove_all_groups(group.field())
            .ok_or(NativeError::FieldConvertError(
                "This fields doesnt exist.".to_string(),
            ))
    }

    /// Check if any groups of this type exist
    fn has_group(&self, group: &Group) -> bool {
        self.get_field_map().has_group(group.field())
    }

    /// check if group exists at specific index
    fn has_group_at(&self, num: usize, group: &Group) -> bool {
        self.get_field_map().has_group_at(num, group.field())
    }
}

impl GroupOperations for Group {
    fn get_field_map(&self) -> &FieldMap {
        &self.fieldmap
    }
    fn get_field_map_mut(&mut self) -> &mut FieldMap {
        &mut self.fieldmap
    }
}

impl Group {
    pub fn new(field_id: u32, delim: u32, order: Option<Vec<u32>>) -> Self {
        let msg_order = if let Some(ord) = order {
            MessageOrder::group(ord)
        } else {
            MessageOrder::normal()
        };
        Self {
            fieldmap: FieldMap::new_with_order(msg_order),
            field_id,
            delim,
        }
    }

    //TODO: other constructors in group.h

    pub fn field(&self) -> u32 {
        self.field_id
    }
    pub fn delim(&self) -> u32 {
        self.delim
    }
    /// Get group count for a specific field
    pub fn group_count(&self, group: &Group) -> usize {
        self.fieldmap.group_count(group.field())
    }

    pub fn set_field(&mut self, tag: u32, value: String) {
        self.fieldmap.set_field(tag, value);
    }

    pub fn get_field(&self, tag: u32) -> Option<&String> {
        self.fieldmap.get_field(tag)
    }

    pub fn has_field(&self, tag: u32) -> bool {
        self.fieldmap.has_field(tag)
    }

    pub fn remove_field(&mut self, tag: u32) -> Option<String> {
        self.fieldmap.remove_field(tag)
    }

    pub fn set_string(&mut self, tag: u32, value: &str) {
        self.fieldmap.set_string(tag, value);
    }

    pub fn get_string(&self, tag: u32) -> Result<String> {
        self.fieldmap.get_string(tag)
    }

    pub fn set_int(&mut self, tag: u32, value: u32) {
        self.fieldmap.set_int(tag, value);
    }

    pub fn get_int(&self, tag: u32) -> Result<u32> {
        self.fieldmap.get_int(tag)
    }

    pub fn set_double(&mut self, tag: u32, value: f64) {
        self.fieldmap.set_float(tag, value);
    }

    pub fn get_double(&self, tag: u32) -> Result<f64> {
        self.fieldmap.get_float(tag)
    }

    pub fn set_bool(&mut self, tag: u32, value: bool) {
        self.fieldmap.set_bool(tag, value);
    }

    pub fn get_bool(&self, tag: u32) -> Result<bool> {
        self.fieldmap.get_bool(tag)
    }

    pub fn set_char(&mut self, tag: u32, value: char) {
        self.fieldmap.set_char(tag, value);
    }

    pub fn get_char(&self, tag: u32) -> Result<char> {
        self.fieldmap.get_char(tag)
    }

    /// convert to FIX string format
    pub fn to_fix_string(&self) -> String {
        self.fieldmap.to_fix_string()
    }

    /// parse from FIX string format
    pub fn from_fix_string(&mut self, fix_string: &str) -> Result<()> {
        //TODO:make sure this one matches the c++ codebase. for now it should work!
        self.fieldmap.from_fix_string(fix_string)
    }

    /// get the underlying FieldMap
    pub fn get_field_map(&self) -> &FieldMap {
        &self.fieldmap
    }

    /// get mutable reference to underlying FieldMap
    pub fn get_field_map_mut(&mut self) -> &mut FieldMap {
        &mut self.fieldmap
    }
}

impl std::ops::Deref for Group {
    type Target = FieldMap;

    fn deref(&self) -> &Self::Target {
        &self.fieldmap
    }
}

impl std::ops::DerefMut for Group {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fieldmap
    }
}
