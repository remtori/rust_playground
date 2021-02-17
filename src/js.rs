use std::*;
use crate::*;

pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Rational(f64),
    Integer(i32),
    BigInt(()),
    String(String),
    Object(()),
    Symbol(()),
}

pub enum PreferredType {
    None,
    Number,
    String,
}

impl ast::ASTNode for Value {
    fn eval(&mut self, _: &mut Context) -> Value {
        *self
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Undefined => write!(f, "Value{{undefined}}"),
            Value::Null => write!(f, "Value{{null}}"),
            Value::Boolean(b) => write!(f, "Value{{bool: {}}}", b),
            Value::Rational(v) => write!(f, "Value{{number: {}}}", v),
            Value::Integer(v) => write!(f, "Value{{number: {}}}", v),
            Value::String(s) => write!(f, "Value{{string: {}}}", s),
            Value::Object(_) => write!(f, "Value{{object}}"),
            Value::BigInt(_) => write!(f, "Value{{bigint}}"),
            Value::Symbol(_) => write!(f, "Value{{symbol}}"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl Value {

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, Value::Undefined)
    }

    pub fn is_bigint(&self) -> bool {
        matches!(self, Value::BigInt(_))
    }

    pub fn is_nan(&self) -> bool {
        if let Value::Rational(v) = self {
            v.is_nan()
        } else {
            false
        }
    }

    pub fn number_from_str(str: &str) -> Value {
        if let Ok(v) = str.parse::<i32>() {
            Value::Integer(v)
        } else if let Ok(v) = str.parse::<f64>() {
            Value::Rational(v)
        } else {
            Value::Rational(f64::NAN)
        }
    }

    /// JS Value type conversion
    pub fn to_primitive(&self, _preferred_type: PreferredType) -> Value {
        if let Value::Object(_obj) = self {
            todo!()
        } else {
            *self
        }
    }

    pub fn to_boolean(&self) -> bool {
        match &self {
            Value::Undefined => false,
            Value::Null => false,
            Value::Boolean(v) => *v,
            Value::Rational(v) => !(v.is_nan() && *v == 0.0),
            Value::Integer(v) => *v == 0,
            Value::BigInt(_) => todo!(),
            Value::String(s) => !s.is_empty(),
            Value::Object(_) => true,
        }
    }

    pub fn to_number(&self) -> Value {
        match self {
            Value::Undefined => Value::Rational(f64::NAN),
            Value::Null => Value::Integer(0),
            Value::Boolean(b) => Value::Integer(if *b { 1 } else { 0 }),
            Value::Rational(_) => *self,
            Value::Integer(_) => *self,
            Value::BigInt(_) => todo!("TypeError"),
            Value::Symbol(_) => todo!("TypeError"),
            Value::String(s) => Value::Rational(*s.parse::<f64>().ok().get_or_insert(f64::NAN)),
            Value::Object(_) => self.to_primitive(PreferredType::Number).to_number(),
        }
    }

    pub fn to_numeric(&self) -> Value {
        let primitive_value = self.to_primitive(PreferredType::Number);
        if primitive_value.is_bigint() {
            primitive_value
        } else {
            primitive_value.to_number()
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Undefined => "undefined".to_owned(),
            Value::Null => "null".to_owned(),
            Value::Boolean(v) => v.to_string(),
            Value::Rational(v) => v.to_string(),
            Value::Integer(v) => v.to_string(),
            Value::BigInt(_) => todo!(),
            Value::String(v) => *v,
            Value::Object(_) => self.to_primitive(PreferredType::String).to_string(),
            Value::Symbol(_) => todo!("TypeError"),
        }
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Rational(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Integer(v)
    }
}
