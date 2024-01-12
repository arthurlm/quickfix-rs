use quickfix_spec_parser::FieldSpec;

pub struct FixCodeSpec {
    pub field_specs: Vec<FieldSpec>,
    pub headers: Vec<SubComponent>,
    pub trailers: Vec<SubComponent>,
    pub messages: Vec<MessageSpec>,
}

pub struct MessageSpec {
    pub name: String,
    pub msg_type: String,
    pub components: Vec<SubComponent>,
}

pub enum SubComponent {
    Field(MessageField),
    Group(MessageGroup),
}

pub struct MessageField {
    pub name: String,
    pub required: bool,
}

pub struct MessageGroup {
    pub name: String,
    pub components: Vec<SubComponent>,
}

impl SubComponent {
    pub fn name(&self) -> &str {
        match self {
            SubComponent::Field(x) => &x.name,
            SubComponent::Group(x) => &x.name,
        }
    }

    pub fn is_required(&self) -> bool {
        // API is a little bit hard to define if we include required group as required parameters 😢.
        matches!(self, Self::Field(x) if x.required)
    }
}
