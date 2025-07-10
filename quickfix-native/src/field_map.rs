use std::collections::{BTreeMap, HashMap};

use crate::{
    errors::{NativeError, Result},
    group::Group,
    message_order::MessageOrder,
};

#[derive(Debug, Clone)]
pub struct FieldMap {
    /// tag->value
    pub fields: BTreeMap<i32, String>,
    /// tag->Group
    pub groups: HashMap<i32, Vec<Group>>,
    /// expected field_order
    pub message_order: MessageOrder,
}

impl FieldMap {
    pub fn new() -> Self {
        FieldMap {
            fields: BTreeMap::new(),
            groups: HashMap::new(),
            message_order: MessageOrder::normal(),
        }
    }

    pub fn new_with_order(order: MessageOrder) -> Self {
        FieldMap {
            fields: BTreeMap::new(),
            groups: HashMap::new(),
            message_order: order,
        }
    }
    //----Basic field ops

    pub fn set_field(&mut self, tag: i32, value: String) {
        self.fields.insert(tag, value);
    }

    pub fn get_field(&self, tag: i32) -> Option<&String> {
        self.fields.get(&tag)
    }

    pub fn has_field(&self, tag: i32) -> bool {
        self.fields.contains_key(&tag)
    }

    pub fn remove_field(&mut self, tag: i32) -> Option<String> {
        self.fields.remove(&tag)
    }

    pub fn get_field_tags(&self) -> Vec<i32> {
        self.fields.keys().copied().collect()
    }

    pub fn clear(&mut self) {
        self.fields.clear();
        self.groups.clear();
    }
    pub fn size(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    //----type safe field ops

    /// get a field as string
    pub fn get_string(&self, tag: i32) -> Result<String> {
        self.get_field(tag)
            .ok_or_else(|| NativeError::FieldNotFound(format!("Field {} not found", tag)))
            .map(|s| s.to_string())
    }

    /// set a field as string
    pub fn set_string(&mut self, tag: i32, value: &str) {
        self.set_field(tag, value.to_string());
    }

    /// get a field as i32
    pub fn get_int(&self, tag: i32) -> Result<i32> {
        let ret = self.get_string(tag)?;
        ret.parse::<i32>().map_err(|_| {
            NativeError::FieldConvertError(format!("failed to convert field {} to i32", tag))
        })
    }

    /// set a field as i32
    pub fn set_int(&mut self, tag: i32, value: i32) {
        self.set_field(tag, value.to_string());
    }

    /// get a field as f64
    pub fn get_float(&self, tag: i32) -> Result<f64> {
        let ret = self.get_string(tag)?;
        ret.parse::<f64>().map_err(|_| {
            NativeError::FieldConvertError(format!("failed to convert field {} to f64", tag))
        })
    }

    /// set a field as f64
    pub fn set_float(&mut self, tag: i32, value: f64) {
        self.set_field(tag, value.to_string());
    }

    /// get a field as boolean (y/n)
    pub fn get_bool(&self, tag: i32) -> Result<bool> {
        let value = self.get_string(tag)?;
        match value.to_uppercase().as_str() {
            "Y" | "YES" | "TRUE" => Ok(true),
            "N" | "NO" | "FALSE" => Ok(false),
            _ => Err(NativeError::FieldConvertError(format!(
                "cannot convert {} to bool",
                value
            ))),
        }
    }

    /// set a field as bool
    pub fn set_bool(&mut self, tag: i32, value: bool) {
        let value = if value { "Y" } else { "N" };
        self.set_string(tag, value);
    }

    /// get a field as char
    pub fn get_char(&self, tag: i32) -> Result<char> {
        let value = self.get_string(tag)?;
        value
            .chars()
            .next()
            .ok_or_else(|| NativeError::FieldConvertError(format!("field {} is empty", tag)))
    }

    /// set field as char
    pub fn set_char(&mut self, tag: i32, value: char) {
        self.set_string(tag, &value.to_string());
    }
    //----Group ops

    /// add a group
    pub fn add_group(&mut self, tag: i32, group: Group) {
        self.groups.entry(tag).or_insert_with(Vec::new).push(group);
        let count = self.groups.get(&tag).map(|v| v.len()).unwrap_or(0);
        self.set_int(tag, count as i32);
    }
    /// get groups for a tag
    pub fn get_groups(&self, tag: i32) -> Option<&Vec<Group>> {
        self.groups.get(&tag)
    }
    /// get mutable groups for a tag
    pub fn get_groups_mut(&mut self, tag: i32) -> Option<&mut Vec<Group>> {
        self.groups.get_mut(&tag)
    }
    /// get a specific group for a tag by index
    pub fn get_group(&self, tag: i32, index: usize) -> Option<&Group> {
        self.groups.get(&tag)?.get(index)
    }

    /// get a mutable specific group for a tag by index
    pub fn get_group_mut(&mut self, tag: i32, index: usize) -> Option<&mut Group> {
        self.groups.get_mut(&tag)?.get_mut(index)
    }

    /// replace a group at a specific index
    pub fn replace_group(&mut self, tag: i32, index: usize, group: Group) -> Result<()> {
        let groups = self
            .get_groups_mut(tag)
            .ok_or_else(|| NativeError::FieldNotFound(format!("no groups for tag {}", tag)))?;

        if index >= groups.len() {
            return Err(NativeError::FieldNotFound(format!(
                "group index {} out of bound",
                index
            )));
        }
        groups[index] = group;
        Ok(())
    }
    /// remove group at a specific index
    pub fn remove_group(&mut self, tag: i32, index: usize) -> Result<Group> {
        let groups = self
            .get_groups_mut(tag)
            .ok_or_else(|| NativeError::FieldNotFound(format!("no groups for tag {}", tag)))?;

        if index >= groups.len() {
            return Err(NativeError::FieldNotFound(format!(
                "group index {} out of bound",
                index
            )));
        }
        let group = groups.remove(index);

        let count = groups.len();

        if count == 0 {
            self.remove_field(tag);
            self.groups.remove(&tag);
        } else {
            self.set_int(tag, count as i32);
        }
        Ok(group)
    }

    /// remove all groups for a tag
    pub fn remove_all_groups(&mut self, tag: i32) -> Option<Vec<Group>> {
        self.remove_field(tag);
        self.groups.remove(&tag)
    }

    /// checks if  group exists for a tag
    pub fn has_group(&self, tag: i32) -> bool {
        self.groups.contains_key(&tag) && !self.groups[&tag].is_empty()
    }

    /// group count for a tag
    pub fn group_count(&self, tag: i32) -> usize {
        self.groups.get(&tag).map(|v| v.len()).unwrap_or(0)
    }

    //----Serializations

    /// convert to FIX string format
    pub fn to_fix_string(&self) -> String {
        let mut results = String::new();
        let mut field_tags: Vec<i32> = self.fields.keys().copied().collect();
        field_tags.sort_by(|&a, &b| self.message_order.compare(a, b));

        for tag in field_tags {
            if let Some(value) = self.fields.get(&tag) {
                results.push_str(&format!("{}={}\x01", tag, value));
            }
        }

        for (&group_tag, groups) in &self.groups {
            for group in groups {
                results.push_str(&group.to_fix_string());
            }
        }
        results
    }

    /// parse a FIX format from a string
    pub fn from_fix_string(&mut self, fix_String: &str) -> Result<()> {
        self.clear();
        let fields: Vec<&str> = fix_String.split('\x01').collect();

        for field_str in fields {
            if field_str.is_empty() {
                continue;
            }

            let parts: Vec<&str> = field_str.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(NativeError::InvalidMessage(format!(
                    "Invalid field format: {}",
                    field_str
                )));
            }
            let tag = parts[0]
                .parse::<i32>()
                .map_err(|_| NativeError::InvalidMessage(format!("Invalid tag: {}", parts[0])))?;

            let value = parts[1].to_string();
            self.set_field(tag, value);
        }
        Ok(())
    }

    //----Helpers

    // copy fields from another FieldMap
    pub fn copy_from(&mut self, other: &FieldMap) {
        for (tag, value) in &other.fields {
            self.set_field(*tag, value.clone());
        }

        for (tag, groups) in &other.groups {
            for group in groups {
                self.add_group(*tag, group.clone());
            }
        }
    }

    /// get the message order
    pub fn get_message_order(&self) -> &MessageOrder {
        &self.message_order
    }

    /// set the message order
    pub fn set_message_order(&mut self, order: MessageOrder) {
        self.message_order = order;
    }
}

impl Default for FieldMap {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a FieldMap {
    type Item = (&'a i32, &'a String);
    type IntoIter = std::collections::btree_map::Iter<'a, i32, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter()
    }
}
