use std::collections::HashMap;

use crate::{
    errors::{NativeError, Result},
    group::Group,
    message_order::MessageOrder,
};

//TODO: Many methods could be abstracted away behind generics!

pub const SOH: char = '\x01';

#[derive(Debug, Clone)]
pub struct Field {
    pub tag: u32,
    pub value: String,
}

impl Field {
    pub fn new(tag: u32, value: String) -> Self {
        Self { tag, value }
    }

    pub fn to_fix_string(&self) -> String {
        format!("{}={}{}", self.tag, self.value, SOH)
    }

    pub fn length(&self) -> usize {
        self.to_fix_string().len()
    }

    pub fn checksum(&self) -> u32 {
        self.to_fix_string().bytes().map(|b| b as u32).sum()
    }

    pub fn parse_from_fix_string(fix_string: &str, pos: usize) -> Result<(Field, usize)> {
        let remaining_str = &fix_string[pos..];
        let eq_pos = remaining_str.find('=').ok_or_else(|| {
            NativeError::InvalidMessage("No equal sign found in fix string".to_string())
        })?;

        let tag = remaining_str[..eq_pos]
            .parse::<u32>()
            .map_err(|_| NativeError::InvalidMessage("Invalid tag value".to_string()))?;

        let soh_pos = remaining_str[eq_pos + 1usize..].find(SOH).ok_or_else(|| {
            NativeError::InvalidMessage("failed to find SOH after value".to_string())
        })?;

        let value = remaining_str[eq_pos + 1..eq_pos + 1 + soh_pos].to_string();

        // soh_pos is relative to eq_pos so we must consider it when using the complete string
        Ok((Field { tag, value }, pos + (eq_pos + 1) + soh_pos + 1))
    }
}

#[derive(Debug, Clone)]
pub struct FieldMap {
    /// tag->value
    /// since we might have duplicate/repeating groups with the same tag value,
    /// we should use Vec which can allow us to store all occurances
    pub fields: Vec<Field>,
    /// tag->Group
    pub groups: HashMap<u32, Vec<Group>>,
    /// expected field_order
    pub message_order: MessageOrder,
}

impl FieldMap {
    pub fn new() -> Self {
        FieldMap {
            fields: Vec::new(),
            groups: HashMap::new(),
            message_order: MessageOrder::normal(),
        }
    }

    pub fn new_with_order(order: MessageOrder) -> Self {
        FieldMap {
            fields: Vec::new(),
            groups: HashMap::new(),
            message_order: order,
        }
    }
    //----Basic field ops

    pub fn set_field(&mut self, tag: u32, value: String) {
        self.fields.retain(|f| f.tag != tag);
        self.fields.push(Field::new(tag, value));
    }

    pub fn get_field(&self, tag: u32) -> Option<&String> {
        self.fields.iter().find(|t| t.tag == tag).map(|f| &f.value)
    }

    pub fn has_field(&self, tag: u32) -> bool {
        self.fields.iter().any(|t| t.tag == tag)
    }

    pub fn append_field(&mut self, field: Field) {
        self.fields.push(field);
    }
    pub fn remove_field(&mut self, tag: u32) -> Option<String> {
        if let Some(field) = self.fields.iter().position(|f| f.tag == tag) {
            Some(self.fields.remove(field).value)
        } else {
            None
        }
    }

    pub fn get_field_tags(&self) -> Vec<u32> {
        self.fields.iter().map(|f| f.tag).collect()
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
    pub fn get_string(&self, tag: u32) -> Result<String> {
        self.get_field(tag)
            .ok_or_else(|| NativeError::FieldNotFound(format!("Field {tag} not found")))
            .map(|s| s.to_string())
    }

    /// set a field as string
    pub fn set_string<T: ToString>(&mut self, tag: u32, value: T) {
        self.set_field(tag, value.to_string());
    }

    /// get a field as u32
    pub fn get_int(&self, tag: u32) -> Result<u32> {
        let ret = self.get_string(tag)?;
        ret.parse::<u32>().map_err(|_| {
            NativeError::FieldConvertError(format!("failed to convert field {tag} to i32"))
        })
    }

    /// set a field as u32
    pub fn set_int(&mut self, tag: u32, value: u32) {
        self.set_field(tag, value.to_string());
    }

    /// get a field as f64
    pub fn get_float(&self, tag: u32) -> Result<f64> {
        let ret = self.get_string(tag)?;
        ret.parse::<f64>().map_err(|_| {
            NativeError::FieldConvertError(format!("failed to convert field {tag} to f64"))
        })
    }

    /// set a field as f64
    pub fn set_float(&mut self, tag: u32, value: f64) {
        self.set_field(tag, value.to_string());
    }

    /// get a field as boolean (y/n)
    pub fn get_bool(&self, tag: u32) -> Result<bool> {
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
    pub fn set_bool(&mut self, tag: u32, value: bool) {
        let value = if value { "Y" } else { "N" };
        self.set_string(tag, value);
    }

    /// get a field as char
    pub fn get_char(&self, tag: u32) -> Result<char> {
        let value = self.get_string(tag)?;
        value
            .chars()
            .next()
            .ok_or_else(|| NativeError::FieldConvertError(format!("field {tag} is empty")))
    }

    /// set field as char
    pub fn set_char(&mut self, tag: u32, value: char) {
        self.set_string(tag, value);
    }

    pub fn sort_fields(&mut self) {
        self.fields
            .sort_by(|a, b| self.message_order.compare(a.tag, b.tag));
    }

    //----Group ops

    /// add a group
    pub fn add_group(&mut self, group: Group) {
        let field_tag = group.field();

        let groups = self.groups.entry(field_tag).or_default();
        groups.push(group);

        let count = groups.len() as u32;

        self.set_int(field_tag, count);
    }
    /// get groups for a tag
    pub fn get_groups(&self, tag: u32) -> Option<&Vec<Group>> {
        self.groups.get(&tag)
    }
    /// get mutable groups for a tag
    pub fn get_groups_mut(&mut self, tag: u32) -> Option<&mut Vec<Group>> {
        self.groups.get_mut(&tag)
    }
    /// get a specific group for a tag by index
    pub fn get_group(&self, tag: u32, index: usize) -> Option<&Group> {
        self.groups.get(&tag)?.get(index)
    }

    /// get a mutable specific group for a tag by index
    pub fn get_group_mut(&mut self, tag: u32, index: usize) -> Option<&mut Group> {
        self.groups.get_mut(&tag)?.get_mut(index)
    }

    /// replace a group at a specific index
    pub fn replace_group(&mut self, tag: u32, index: usize, group: Group) -> Result<()> {
        let groups = self
            .get_groups_mut(tag)
            .ok_or_else(|| NativeError::FieldNotFound(format!("no groups for tag {tag}")))?;

        if index >= groups.len() {
            return Err(NativeError::FieldNotFound(format!(
                "group index {index} out of bound"
            )));
        }
        groups[index] = group;
        Ok(())
    }

    /// remove group at a specific index
    pub fn remove_group(&mut self, tag: u32, index: usize) -> Result<Group> {
        let groups = self
            .get_groups_mut(tag)
            .ok_or_else(|| NativeError::FieldNotFound(format!("no groups for tag {tag}")))?;

        if index >= groups.len() {
            return Err(NativeError::FieldNotFound(format!(
                "group index {index} out of bound"
            )));
        }
        let group = groups.remove(index);

        let count = groups.len();

        if count == 0 {
            self.remove_field(tag);
            self.groups.remove(&tag);
        } else {
            self.set_int(tag, count as u32);
        }
        Ok(group)
    }

    /// remove all groups for a tag
    pub fn remove_all_groups(&mut self, tag: u32) -> Option<Vec<Group>> {
        self.remove_field(tag);
        self.groups.remove(&tag)
    }

    /// checks if  group exists for a tag
    pub fn has_group(&self, tag: u32) -> bool {
        self.groups.contains_key(&tag) && !self.groups[&tag].is_empty()
    }

    /// checks if a groups at a specific index exsits for a tag
    pub fn has_group_at(&self, num: usize, tag: u32) -> bool {
        if self.get_group(tag, num).is_some() {
            return true;
        }
        false
    }

    /// group count for a tag
    pub fn group_count(&self, tag: u32) -> usize {
        self.groups.get(&tag).map(|v| v.len()).unwrap_or(0)
    }

    //----Serializations

    /// convert to FIX string format
    pub fn to_fix_string(&self) -> String {
        let mut results = String::new();
        for field in &self.fields {
            results.push_str(&field.to_fix_string());

            if let Some(groups) = self.groups.get(&field.tag) {
                for group in groups {
                    results.push_str(&group.to_fix_string());
                }
            }
        }
        results
    }

    /// parse a FIX format from a string
    pub fn from_fix_string(&mut self, fix_string: &str) -> Result<()> {
        self.clear();
        let mut pos = 0;
        while pos < fix_string.len() {
            let (field, new_pos) = Field::parse_from_fix_string(fix_string, pos)?;
            self.append_field(field);
            pos = new_pos;
        }
        self.sort_fields();
        Ok(())
    }

    //----Helpers

    // copy fields from another FieldMap
    pub fn copy_from(&mut self, other: &FieldMap) {
        for field in &other.fields {
            self.set_field(field.tag, field.value.clone());
        }

        for groups in other.groups.values() {
            for group in groups {
                self.add_group(group.clone());
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

    pub fn calculate_length(&self) -> usize {
        let mut length = 0;

        for field in &self.fields {
            length += field.length();
        }

        for groups in &self.groups {
            for group in groups.1 {
                length += group.calculate_length();
            }
        }
        length
    }

    pub fn calculate_checksum(&self) -> u32 {
        let mut checksum = 0;

        for field in &self.fields {
            checksum += field.checksum();
        }

        for groups in self.groups.values() {
            for group in groups {
                checksum += group.calculate_checksum();
            }
        }

        checksum
    }
}

impl Default for FieldMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_field_serialization() {
        let field = Field::new(35, "D".to_string());
        assert_eq!(&field.to_fix_string(), "35=D\x01")
    }

    #[test]
    fn test_field_parsing() {
        let fix_str = "35=D\x01";
        let (field, pos) = Field::parse_from_fix_string(fix_str, 0).unwrap();

        assert_eq!(field.tag, 35);
        assert_eq!(field.value, "D");
        assert_eq!(pos, 5);
    }

    #[test]
    fn test_fieldmap_serialization() {
        let mut field_map = FieldMap::new();

        field_map.set_string(35, "D");
        field_map.set_int(49, 1234);
        field_map.set_float(44, 99.50);

        let fix_str = field_map.to_fix_string();
        assert!(fix_str.contains("35=D\x01"));
        assert!(fix_str.contains("49=1234\x01"));
        assert!(fix_str.contains("44=99.5\x01"));
    }

    #[test]
    fn test_fieldmap_parsing() {
        let fix_string = "35=D\x0149=1234\x0144=99.5\x01";
        let mut fieldmap = FieldMap::new();
        fieldmap.from_fix_string(fix_string).unwrap();
        assert_eq!(fieldmap.get_string(35).unwrap(), "D");
        assert_eq!(fieldmap.get_int(49).unwrap(), 1234);
        assert_eq!(fieldmap.get_float(44).unwrap(), 99.5);
    }

    #[test]
    fn test_group_handling() {
        let mut fieldmap = FieldMap::new();

        // symbol, MDEntryType delim
        let mut group1 = Group::new(55, 269, None);
        group1.set_string(55, "MSFT");
        group1.set_float(44, 100.0);

        fieldmap.add_group(group1);
        assert_eq!(fieldmap.group_count(55), 1);
        assert_eq!(fieldmap.get_int(55).unwrap(), 1);

        let group = fieldmap.get_group(55, 0).unwrap();
        assert_eq!(group.get_string(55).unwrap(), "MSFT");
    }
    #[test]
    fn test_length_calculation() {
        let mut fieldmap = FieldMap::new();
        fieldmap.set_string(35, "D");
        fieldmap.set_int(49, 1234);

        let expected_length = "35=D\x01".len() + "49=1234\x01".len();
        assert_eq!(fieldmap.calculate_length(), expected_length);
    }

    #[test]
    fn test_checksum_calculation() {
        let mut fieldmap = FieldMap::new();
        fieldmap.set_string(35, "D");

        let expected_checksum: u32 = "35=D\x01".bytes().map(|b| b as u32).sum();
        assert_eq!(fieldmap.calculate_checksum(), expected_checksum);
    }
}
