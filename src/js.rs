use crate::*;

#[derive(Debug, Clone)]
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

impl ast::Expression for Value {
    fn eval(&mut self, _: &mut Context) -> Value {
        self.clone()
    }
}

impl Value {
    /// JS Value type conversion
    pub fn to_primitive(&self, _: PreferredType) -> Value {
        if let Value::Object(_) = self {
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
            Value::String(s) => {
                Value::Number(if let Ok(v) = s.parse::<f64>() { v } else { f64::NAN })
            }
            Value::Object(_) => todo!(),
        }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            Value::Undefined => "undefined".to_owned(),
            Value::Null => "null".to_owned(),
            Value::Boolean(v) => v.to_string(),
            Value::Number(v) => v.to_string(),
            Value::String(v) => v.clone(),
            Value::Object(_) => "[object]".to_owned(),
        }
    }
}
