use crate::{
    errors::{NativeError, Result},
    field_map::FieldMap,
    message::Message,
};
use quickfix_spec_parser::{ComponentSpec, FieldType, FieldValue, FixSpec};
use std::collections::{HashMap, HashSet};

//const for user defined fields (FIELD::UserMin)
pub const USER_DEFINED_FIELD_MIN: u32 = 5000;

#[derive(Debug, Clone)]
pub struct DataDictionary {
    //----version info
    version: Option<String>,
    has_version: bool,

    //----validation flags
    check_fields_out_of_order: bool,
    check_fields_have_values: bool,
    check_user_defined_fields: bool,
    allow_unknown_message_fields: bool,
    store_msg_fields_order: bool,

    //----field definitions
    /// valid fields
    fields: HashSet<u32>,
    /// field number-> field name
    field_names: HashMap<u32, String>,
    /// name -> field number
    name_to_field: HashMap<String, u32>,
    /// field number -> type
    field_types: HashMap<u32, FieldType>,
    /// field number -> valid values
    field_values: HashMap<u32, HashSet<String>>,
    /// (field,value) -> description
    value_names: HashMap<(u32, String), String>,

    //----message structure
    /// valid message types
    message_types: HashSet<String>,
    /// msgtype -> valid fields
    message_fields: HashMap<String, HashSet<u32>>,
    /// msgtype -> required fields
    required_fields: HashMap<String, HashSet<u32>>,
    /// field->is required
    header_fields: HashMap<u32, bool>,
    /// field -> is required
    trailer_fields: HashMap<u32, bool>,

    //----field ordering - to preserve msg field order
    ordered_fields: Vec<u32>,
    header_ordered_fields: Vec<u32>,
    trailer_ordered_fields: Vec<u32>,
    message_ordered_fields: HashMap<String, Vec<u32>>,

    //----groups/repeating field structures
    /// field -> (msgtype -> (delim, group_dd))
    groups: HashMap<u32, HashMap<String, (u32, DataDictionary)>>,

    /// fields that have associated length fields
    data_fields: HashSet<u32>,
}

impl DataDictionary {
    pub fn new() -> Self {
        DataDictionary {
            version: None,
            has_version: false,
            check_fields_out_of_order: true,
            check_fields_have_values: true,
            check_user_defined_fields: false,
            allow_unknown_message_fields: false,
            store_msg_fields_order: false,
            fields: HashSet::new(),
            field_names: HashMap::new(),
            name_to_field: HashMap::new(),
            field_types: HashMap::new(),
            field_values: HashMap::new(),
            value_names: HashMap::new(),
            message_types: HashSet::new(),
            message_fields: HashMap::new(),
            required_fields: HashMap::new(),
            header_fields: HashMap::new(),
            trailer_fields: HashMap::new(),
            ordered_fields: Vec::new(),
            header_ordered_fields: Vec::new(),
            trailer_ordered_fields: Vec::new(),
            message_ordered_fields: HashMap::new(),
            groups: HashMap::new(),
            data_fields: HashSet::new(),
        }
    }
    pub fn from_xml_file(path: &str) -> Result<Self> {
        // this will read FIX XML specification files like FIX44.xml
        let xml_content = std::fs::read(path).map_err(NativeError::IoError)?;
        Self::from_xml_bytes(&xml_content)
    }

    pub fn from_xml_string(xml: &str) -> Result<Self> {
        Self::from_xml_bytes(xml.as_bytes())
    }

    pub fn from_xml_bytes(xml: &[u8]) -> Result<Self> {
        let spec = quickfix_spec_parser::parse_spec(xml)
            .map_err(|e| NativeError::XMLError(e.to_string()))?;
        Self::from_fix_spec(spec)
    }

    pub fn from_fix_spec(specs: FixSpec) -> Result<Self> {
        let mut dd = DataDictionary::new();
        let version_str = if specs.is_fixt {
            format!("FIXT.{}.{}", specs.version.0, specs.version.1)
        } else {
            format!("FIX.{}.{}", specs.version.0, specs.version.1)
        };

        dd.set_version(&version_str);

        for field_spec in &specs.field_specs {
            dd.add_field(field_spec.number);
            dd.add_field_name(field_spec.number, &field_spec.name)?;
            dd.add_field_type(field_spec.number, field_spec.r#type);

            for value in &field_spec.values {
                dd.add_field_value(field_spec.number, &value.value);
                dd.add_value_name(field_spec.number, &value.value, &value.description);
            }
        }
        let mut component_map: HashMap<String, &ComponentSpec> = HashMap::new();
        for component in &specs.component_specs {
            component_map.insert(component.name.clone(), component);
        }

        for field_value in &specs.headers {
            dd.process_field_value(field_value, "_header_", &component_map)?;
        }
        for field_value in &specs.trailers {
            dd.process_field_value(field_value, "_trailer_", &component_map)?;
        }

        for message in &specs.messages {
            dd.add_msg_type(&message.msg_type);
            // adding message name as a value for MsgType field (35)
            dd.add_value_name(35, &message.msg_type, &message.name);

            for field_value in &message.values {
                dd.process_field_value(field_value, &message.msg_type, &component_map)?;
            }
        }

        Ok(dd)
    }

    fn process_field_value(
        &mut self,
        field_value: &FieldValue,
        context: &str,
        component_map: &HashMap<String, &ComponentSpec>,
    ) -> Result<()> {
        match field_value {
            FieldValue::Field(field) => {
                let field_num = self
                    .name_to_field
                    .get(&field.name)
                    .copied()
                    .ok_or_else(|| {
                        NativeError::ConfigError(format!(
                            "Field {} not defined in field section",
                            &field.name
                        ))
                    })?;

                match context {
                    "_header_" => {
                        self.add_header_field(field_num, field.required);
                    }
                    "_trailer_" => {
                        self.add_trailer_field(field_num, field.required);
                    }
                    msg_type => {
                        self.add_msg_field(msg_type, field_num);
                        if field.required {
                            self.add_required_field(msg_type, field_num);
                        }
                    }
                }
            }
            FieldValue::Group(group) => {
                let group_field_num =
                    self.name_to_field
                        .get(&group.name)
                        .copied()
                        .ok_or_else(|| {
                            NativeError::ConfigError(format!(
                                "Group field {} not defined in fields section",
                                group.name
                            ))
                        })?;

                let mut group_dd = DataDictionary::new();
                if let Some(version) = self.get_version() {
                    group_dd.set_version(version.as_str());
                }
                let mut delim = 0;

                for (i, field_value) in group.values.iter().enumerate() {
                    match field_value {
                        FieldValue::Field(field) => {
                            let field_num = self
                                .name_to_field
                                .get(&field.name)
                                .copied()
                                .ok_or_else(|| {
                                    NativeError::ConfigError(format!(
                                        "Field {} not defined in fields section",
                                        field.name
                                    ))
                                })?;

                            group_dd.add_field(field_num);
                            if field.required && group.required {
                                group_dd.add_required_field(context, field_num);
                            }
                            //first field in group is the delimiter
                            if i == 0 {
                                delim = field_num
                            }
                        }
                        _ => {
                            group_dd.process_field_value(field_value, context, component_map)?;
                        }
                    }
                }
                if delim != 0 {
                    self.add_group(context.to_string(), group_field_num, delim, group_dd);
                }

                match context {
                    "_header_" => self.add_header_field(group_field_num, group.required),
                    "_trailer_" => self.add_trailer_field(group_field_num, group.required),
                    msg_type => {
                        self.add_msg_field(msg_type, group_field_num);
                        if group.required {
                            self.add_required_field(msg_type, group_field_num);
                        }
                    }
                }
            }
            FieldValue::Component(component) => {
                if let Some(component_spec) = component_map.get(&component.name) {
                    for field_value in &component_spec.values {
                        //recursively process component fields
                        // component required flag must be used for all its fields
                        let effective_required = component.required;
                        match field_value {
                            FieldValue::Field(field) => {
                                let field_num =
                                    self.name_to_field.get(&field.name).copied().ok_or_else(
                                        || {
                                            NativeError::ConfigError(format!(
                                                "Field {} not defined in fields section",
                                                field.name
                                            ))
                                        },
                                    )?;

                                match context {
                                    "_header_" => {
                                        self.add_header_field(
                                            field_num,
                                            field.required && effective_required,
                                        );
                                    }
                                    "_trailer_" => {
                                        self.add_trailer_field(
                                            field_num,
                                            field.required && effective_required,
                                        );
                                    }
                                    msg_type => {
                                        self.add_msg_field(msg_type, field_num);
                                        if field.required && effective_required {
                                            self.add_required_field(msg_type, field_num);
                                        }
                                    }
                                }
                            }
                            _ => {
                                // recursively process nested components/groups
                                self.process_field_value(field_value, context, component_map)?;
                            }
                        }
                    }
                } else {
                    return Err(NativeError::ConfigError(format!(
                        "Component {} not found in component specifications",
                        component.name
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn validate(&self, message: &Message) -> Result<()> {
        // This would implement the core validation logic
        // similar to the C++ DataDictionary::validate method

        // 1. Check message type
        // 2. Check field order
        // 3. Check required fields
        // 4. Check field formats
        // 5. Check field values
        // 6. Check groups

        todo!("Implement full message validation")
    }

    pub fn set_version(&mut self, version: &str) {
        self.version = Some(version.to_string());
        self.has_version = true;
    }

    pub fn set_has_version(&mut self, has_version: bool) {
        self.has_version = has_version;
    }

    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn add_field(&mut self, field: u32) {
        self.fields.insert(field);
        self.ordered_fields.push(field);
    }

    pub fn add_field_name(&mut self, field: u32, name: &str) -> Result<()> {
        if self.name_to_field.contains_key(name) {
            return Err(NativeError::ConfigError(format!(
                "Field name {} defined multiple times",
                name
            )));
        }

        self.name_to_field.insert(name.to_string(), field);
        self.field_names.insert(field, name.to_string());
        Ok(())
    }

    pub fn add_field_type(&mut self, field: u32, field_type: FieldType) {
        self.field_types.insert(field, field_type);

        if matches!(field_type, FieldType::Data | FieldType::XmlData) {
            self.data_fields.insert(field);
        }
    }

    pub fn add_field_value(&mut self, field: u32, value: &str) {
        self.field_values
            .entry(field)
            .or_default()
            .insert(value.to_string());
    }

    pub fn add_value_name(&mut self, field: u32, value: &str, name: &str) {
        self.value_names
            .insert((field, value.to_string()), name.to_string());
    }

    pub fn add_msg_type(&mut self, msg_type: &str) {
        self.message_types.insert(msg_type.to_string());
    }

    pub fn add_msg_field(&mut self, msg_type: &str, field: u32) {
        if self.store_msg_fields_order {
            self.message_ordered_fields
                .entry(msg_type.to_string())
                .or_default()
                .push(field);
        }

        self.message_fields
            .entry(msg_type.to_string())
            .or_default()
            .insert(field);
    }

    pub fn add_required_field(&mut self, msg_type: &str, field: u32) {
        self.required_fields
            .entry(msg_type.to_string())
            .or_default()
            .insert(field);
    }

    pub fn add_header_field(&mut self, field: u32, required: bool) {
        if self.store_msg_fields_order {
            self.header_ordered_fields.push(field);
        }
        self.header_fields.insert(field, required);
    }

    pub fn add_trailer_field(&mut self, field: u32, required: bool) {
        if self.store_msg_fields_order {
            self.trailer_ordered_fields.push(field);
        }
        self.trailer_fields.insert(field, required);
    }

    pub fn add_group(
        &mut self,
        msg_type: String,
        field: u32,
        delim: u32,
        group_dd: DataDictionary,
    ) {
        self.groups
            .entry(field)
            .or_default()
            .insert(msg_type, (delim, group_dd));
    }

    pub fn is_field(&self, field: u32) -> bool {
        self.fields.contains(&field)
    }

    pub fn is_msg_type(&self, msg_type: &str) -> bool {
        self.message_types.contains(msg_type)
    }

    pub fn is_msg_field(&self, msg_type: &str, field: u32) -> bool {
        self.message_fields
            .get(msg_type)
            .map(|fields| fields.contains(&field))
            .unwrap_or(false)
    }

    pub fn is_header_field(&self, field: u32) -> bool {
        self.header_fields.contains_key(&field)
    }

    pub fn is_trailer_field(&self, field: u32) -> bool {
        self.trailer_fields.contains_key(&field)
    }

    pub fn is_required_field(&self, msg_type: &str, field: u32) -> bool {
        self.required_fields
            .get(msg_type)
            .map(|fields| fields.contains(&field))
            .unwrap_or(false)
    }

    pub fn is_field_value(&self, field: u32, value: &str) -> bool {
        if let Some(valid_values) = self.field_values.get(&field) {
            if self.is_multiple_value_field(field) {
                value.split(' ').all(|v| valid_values.contains(v))
            } else {
                valid_values.contains(value)
            }
        } else {
            true // no restrictions if no value is defined
        }
    }
    pub fn is_multiple_value_field(&self, field: u32) -> bool {
        if let Some(field_type) = self.field_types.get(&field) {
            matches!(
                field_type,
                FieldType::MultipleCharValue
                    | FieldType::MultipleStringValue
                    | FieldType::MultipleValueString
            )
        } else {
            false
        }
    }

    pub fn is_data_field(&self, field: u32) -> bool {
        self.data_fields.contains(&field)
    }

    pub fn get_field_type(&self, field: u32) -> Option<&FieldType> {
        self.field_types.get(&field)
    }

    pub fn get_field_name(&self, field: u32) -> Option<&String> {
        self.field_names.get(&field)
    }

    pub fn get_field_tag(&self, name: &str) -> Option<u32> {
        self.name_to_field.get(name).copied()
    }

    pub fn check_fields_out_of_order(&mut self, value: bool) {
        self.check_fields_out_of_order = value;
    }

    pub fn check_user_defined_fields(&mut self, value: bool) {
        self.check_user_defined_fields = value;
    }

    pub fn check_fields_have_values(&mut self, value: bool) {
        self.check_fields_have_values = value;
    }

    pub fn allow_unknown_tags(&mut self, value: bool) {
        self.allow_unknown_message_fields = value;
    }

    pub fn preserve_message_fields_order(&mut self, value: bool) {
        self.store_msg_fields_order = value;
    }

    pub fn check_valid_format(&self, field: u32, value: &str) -> Result<()> {
        todo!();
    }

    pub fn check_has_value(&self, field: u32, _value: &str) -> Result<()> {
        if !self.is_field(field) {
            return Err(NativeError::InvalidTagNumber(field));
        }
        Ok(())
    }

    pub fn check_field_value(&self, field: u32, value: &str) -> Result<()> {
        if self.field_values.contains_key(&field) && !self.is_field_value(field, value) {
            return Err(NativeError::IncorrectTagValue(field));
        }
        Ok(())
    }

    pub fn check_valid_tag_number(&self, field: u32) -> Result<()> {
        if !self.is_field(field) {
            return Err(NativeError::InvalidTagNumber(field));
        }
        Ok(())
    }

    pub fn check_is_in_message(&self, field: u32, msg_type: &str) -> Result<()> {
        if !self.is_msg_field(msg_type, field) {
            return Err(NativeError::TagNotDefinedforMessage(field));
        }
        Ok(())
    }

    // fn should_check_tag(&self, field: u32) -> bool {
    //     if self.allow_unknown_message_fields && field < USER_DEFINED_FIELD_MIN {
    //         return false;
    //     } else if !self.check_user_defined_fields && field >= USER_DEFINED_FIELD_MIN {
    //         return false;
    //     }
    //     true
    // }

    pub fn check_group_count(
        &self,
        field: u32,
        field_map: &FieldMap,
        msg_type: &str,
    ) -> Result<()> {
        if self.is_group(msg_type, field) {
            // Get the field value first
            if let Some(value) = field_map.get_field(field) {
                let actual_count = field_map.group_count(field);
                let expected_count: usize = value
                    .parse()
                    .map_err(|_| NativeError::IncorrectDataFormat(field, value.to_string()))?;

                if actual_count != expected_count {
                    return Err(NativeError::RepeatingGroupCountMismatch(field));
                }
            }
            // If field value is not set but group exists, that's also an error
            else {
                return Err(NativeError::NoTagValue(field));
            }
        }
        Ok(())
    }
    pub fn get_name_value(&self, field: u32, name: &str) -> Option<String> {
        for ((f, val), desc) in &self.value_names {
            if *f == field && desc == name {
                return Some(val.clone());
            }
        }
        None
    }
    pub fn is_group(&self, msg_type: &str, field: u32) -> bool {
        if let Some(group_map) = self.groups.get(&field) {
            group_map.contains_key(msg_type)
        } else {
            false
        }
    }

    pub fn get_group(&self, msg_type: &str, field: u32) -> Option<(u32, &DataDictionary)> {
        self.groups
            .get(&field)
            .and_then(|group_map| group_map.get(msg_type))
            .map(|(delim, dd)| (*delim, dd))
    }
}

impl Default for DataDictionary {
    fn default() -> Self {
        Self::new()
    }
}
