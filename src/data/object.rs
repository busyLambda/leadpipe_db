use std::collections::HashMap;

use super::_type::Type;

pub struct Object<'a> {
    name: &'a str,
    fields: Vec<(String, Type)>,
    size_in_bits: u64,
}

impl<'a> Object<'a> {
    pub fn fields(&self) -> &Vec<(String, Type)> {
        &self.fields
    }

    pub fn size_in_bits(&self) -> &u64 {
        &self.size_in_bits
    }

    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            fields: Vec::new(),
            size_in_bits: 0,
        }
    }

    pub fn add_field(&mut self, name: String, _type: Type) {
        let field_size = _type.size_in_bits();

        self.size_in_bits += field_size;

        self.fields.push((name, _type));
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
}
