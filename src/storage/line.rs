use std::collections::HashMap;

use super::value::Value;

pub struct Line {
    fields: Vec<(String, Value)>,
}

impl Line {
    pub fn fields(&self) -> &Vec<(String, Value)> {
        &self.fields
    }

    pub fn new() -> Self {
        Self {
            fields: Vec::new()
        }
    }

    pub fn add_field(&mut self, name: String, value: Value) {
        self.fields.push((name, value));
    }

    pub fn to_string(&self) -> String {
        let mut result = String::from("{");

        let len = self.fields().len();
        let mut i = 0;
        for (name, value) in self.fields() {
            result.push_str(format!("{}: {}", name, value.to_string()).as_str());
            
            if i + 1 != len {
                result.push_str(", ")
            }

            i += 1;
        }

        result.push('}');
        
        result
    }
}