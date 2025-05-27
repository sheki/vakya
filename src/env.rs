use crate::expr::Value;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Default)]
pub struct Env {
    values: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: impl Into<String>, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    #[allow(dead_code)]
    pub fn assign(&mut self, name: impl Into<String>, value: Value) -> Result<(), String> {
        let name = name.into();
        match self.values.entry(name) {
            Entry::Occupied(mut e) => {
                e.insert(value);
                Ok(())
            }
            Entry::Vacant(e) => Err(format!("Undefined variable '{}'", e.key())),
        }
    }
}
