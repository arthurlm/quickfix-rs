use crate::{errors::Result, field_map::FieldMap, message_order::MessageOrder};

#[derive(Debug, Clone)]
pub struct Group {
    fieldmap: FieldMap,
    /// group's identifying field tag9
    field_id: i32,
    /// delimiter field that  starts each group entry
    delim: i32,
}

impl Group {
    pub fn new(field_id: i32, delim: i32, order: Option<Vec<i32>>) -> Self {
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

    pub fn field(&self) -> i32 {
        self.field_id
    }
    pub fn delim(&self) -> i32 {
        self.delim
    }
    // pub fn set_field(&mut self, tag: i32, value: String) {
    //     self.fieldmap.set_field(tag, value);
    // }

    /// add a group
    pub fn add_group(&mut self, group: &Group) {
        self.fieldmap.add_group(group.field(), group.clone());
    }
    /// replace a group at a specific index
    pub fn replace_group(&mut self, num: usize, group: &Group) {
        self.fieldmap
            .replace_group(group.field(), num, group.clone());
    }
    /// get a group at a specific group
    pub fn get_group(&self, num: usize, group: &Group) -> Result<Group> {
        self.fieldmap
            .get_group(group.field(), num)
            .ok_or_else(|| {
                crate::errors::NativeError::FieldNotFound(format!(
                    "Group not found at index {} for field {}",
                    num,
                    group.field()
                ))
            })
            .map(|g| g.clone())
    }
    /// remove a group at a specific index
    pub fn remove_group_at(&mut self, num: usize, group: &Group) -> Result<Group> {
        self.fieldmap.remove_group(group.field(), num)
    }
    /// remove all groups of this type
    pub fn remove_group(&mut self, group: &Group) -> Option<Vec<Group>> {
        self.fieldmap.remove_all_groups(group.field())
    }

    /// check if group exists at specific index
    pub fn has_group_at(&self, num: usize, group: &Group) -> bool {
        self.fieldmap.get_group(group.field(), num).is_some()
    }
    /// Check if any groups of this type exist
    pub fn has_group(&self, group: &Group) -> bool {
        self.fieldmap.has_group(group.field())
    }

    /// Get group count for a specific field
    pub fn group_count(&self, group: &Group) -> usize {
        self.fieldmap.group_count(group.field())
    }

    pub fn set_field(&mut self, tag: i32, value: String) {
        self.fieldmap.set_field(tag, value);
    }

    pub fn get_field(&self, tag: i32) -> Option<&String> {
        self.fieldmap.get_field(tag)
    }

    pub fn has_field(&self, tag: i32) -> bool {
        self.fieldmap.has_field(tag)
    }

    pub fn remove_field(&mut self, tag: i32) -> Option<String> {
        self.fieldmap.remove_field(tag)
    }

    pub fn set_string(&mut self, tag: i32, value: &str) {
        self.fieldmap.set_string(tag, value);
    }

    pub fn get_string(&self, tag: i32) -> Result<String> {
        self.fieldmap.get_string(tag)
    }

    pub fn set_int(&mut self, tag: i32, value: i32) {
        self.fieldmap.set_int(tag, value);
    }

    pub fn get_int(&self, tag: i32) -> Result<i32> {
        self.fieldmap.get_int(tag)
    }

    pub fn set_double(&mut self, tag: i32, value: f64) {
        self.fieldmap.set_float(tag, value);
    }

    pub fn get_double(&self, tag: i32) -> Result<f64> {
        self.fieldmap.get_float(tag)
    }

    pub fn set_bool(&mut self, tag: i32, value: bool) {
        self.fieldmap.set_bool(tag, value);
    }

    pub fn get_bool(&self, tag: i32) -> Result<bool> {
        self.fieldmap.get_bool(tag)
    }

    pub fn set_char(&mut self, tag: i32, value: char) {
        self.fieldmap.set_char(tag, value);
    }

    pub fn get_char(&self, tag: i32) -> Result<char> {
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
