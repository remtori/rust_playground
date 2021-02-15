use std::*;

use crate::*;

#[derive(Clone)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(()),
}

pub enum PreferredType {
    None,
    Number,
    String,
}

impl ast::ASTNode for Value {
    fn eval(&mut self, _: &mut Context) -> Value {
        self.clone()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Undefined => write!(f, "Value{{undefined}}"),
            Value::Null => write!(f, "Value{{null}}"),
            Value::Boolean(b) => write!(f, "Value{{bool: {}}}", b),
            Value::Number(v) => write!(f, "Value{{number: {}}}", v),
            Value::String(s) => write!(f, "Value{{string: {}}}", s),
            Value::Object(_) => write!(f, "Value{{object}}"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl Value {
    /// JS Value type conversion
    pub fn to_primitive(&self, _preferred_type: PreferredType) -> Value {
        if let Value::Object(_obj) = self {
            todo!()
        } else {
            self.clone()
        }
    }

    pub fn to_boolean(&self) -> bool {
        match &self {
            Value::Undefined => false,
            Value::Null => false,
            Value::Boolean(v) => *v,
            Value::Number(v) => !(v.is_nan() && *v == 0.0),
            Value::String(s) => !s.is_empty(),
            Value::Object(_) => true,
        }
    }

    pub fn to_number(&self) -> Value {
        match self {
            Value::Undefined => Value::Number(f64::NAN),
            Value::Null => Value::Number(0.0),
            Value::Boolean(b) => Value::Number(if *b { 1.0 } else { 0.0 }),
            Value::Number(v) => Value::Number(*v),
            Value::String(s) => Value::Number(*s.parse::<f64>().ok().get_or_insert(f64::NAN)),
            Value::Object(_) => self.to_primitive(PreferredType::Number).to_number(),
        }
    }

    pub fn spec_to_string(&self) -> String {
        match self {
            Value::Undefined => "undefined".to_owned(),
            Value::Null => "null".to_owned(),
            Value::Boolean(v) => v.to_string(),
            Value::Number(v) => v.to_string(),
            Value::String(v) => v.clone(),
            Value::Object(_) => self.to_primitive(PreferredType::String).spec_to_string(),
        }
    }
}
