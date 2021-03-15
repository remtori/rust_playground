use std::*;

use crate::*;

#[derive(Clone)]
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

// impl ast::ASTNode for Value {
//     fn eval(&mut self, _: &mut Context) -> Value {
//         self.clone()
//     }
// }

impl fmt::Debug for Value {
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

impl Value {
    // Constructors

    pub fn nan() -> Value {
        Value::Rational(f64::NAN)
    }

    pub fn number_from_str(str: &str) -> Value {
        if let Ok(v) = str.parse::<i32>() {
            Value::Integer(v)
        } else if let Ok(v) = str.parse::<f64>() {
            Value::Rational(v)
        } else {
            Value::nan()
        }
    }

    // Utility

    pub fn as_f64(&self) -> f64 {
        match self {
            Value::Rational(v) => *v,
            Value::Integer(v) => *v as f64,
            _ => panic!("Failed to cast {:?} as f64", self),
        }
    }

    pub fn as_i32(&self) -> i32 {
        match self {
            Value::Rational(v) => *v as i32,
            Value::Integer(v) => *v as i32,
            _ => panic!("Failed to cast {:?} as T", self),
        }
    }

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

    pub fn is_infinity(&self) -> bool {
        if let Value::Rational(v) = self {
            v.is_infinite()
        } else {
            false
        }
    }

    // JS Value type conversion

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
            Value::Rational(v) => !(v.is_nan() && *v == 0.0),
            Value::Integer(v) => *v == 0,
            Value::BigInt(_) => todo!(),
            Value::Symbol(_) => true,
            Value::String(s) => !s.is_empty(),
            Value::Object(_) => true,
        }
    }

    pub fn to_number(&self) -> Value {
        match self {
            Value::Undefined => Value::nan(),
            Value::Null => Value::Integer(0),
            Value::Boolean(b) => Value::Integer(if *b { 1 } else { 0 }),
            Value::Rational(_) => self.clone(),
            Value::Integer(_) => self.clone(),
            Value::BigInt(_) => panic!("TypeError"),
            Value::Symbol(_) => panic!("TypeError"),
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

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match &self {
            Value::Undefined => "undefined".to_owned(),
            Value::Null => "null".to_owned(),
            Value::Boolean(v) => v.to_string(),
            Value::Rational(v) => v.to_string(),
            Value::Integer(v) => v.to_string(),
            Value::BigInt(_) => todo!(),
            Value::String(v) => v.clone(),
            Value::Object(_) => self.to_primitive(PreferredType::String).to_string(),
            Value::Symbol(_) => panic!("TypeError"),
        }
    }

    pub fn to_i32(&self) -> i32 {
        let number = self.to_number().as_f64();

        if number.is_infinite() || number.is_nan() || number == 0.0 {
            return 0;
        }

        let mut int_value = number.abs().floor();
        if number.is_sign_negative() {
            int_value = -int_value;
        }

        let mut int32 = int_value % 4294967296.0;
        if int32 > 2147483648.0 {
            int32 -= 4294967296.0;
        }

        int32 as i32
    }

    pub fn to_u32(&self) -> u32 {
        let number = self.to_number().as_f64();

        if number.is_infinite() || number.is_nan() || number == 0.0 {
            return 0;
        }

        let mut int_value = number.abs().floor();
        if number.is_sign_negative() {
            int_value = -int_value;
        }

        (int_value % 4294967296.0) as u32
    }

    pub fn to_i16(&self) -> i16 {
        let number = self.to_number().as_f64();

        if number.is_infinite() || number.is_nan() || number == 0.0 {
            return 0;
        }

        let mut int_value = number.abs().floor();
        if number.is_sign_negative() {
            int_value = -int_value;
        }

        let mut int16 = int_value % 65536.0;
        if int16 > 32768.0 {
            int16 -= 65536.0;
        }

        int16 as i16
    }

    pub fn to_u16(&self) -> u16 {
        let number = self.to_number().as_f64();

        if number.is_infinite() || number.is_nan() || number == 0.0 {
            return 0;
        }

        let mut int_value = number.abs().floor();
        if number.is_sign_negative() {
            int_value = -int_value;
        }

        (int_value % 65536.0) as u16
    }

    pub fn to_i8(&self) -> i8 {
        let number = self.to_number().as_f64();

        if number.is_infinite() || number.is_nan() || number == 0.0 {
            return 0;
        }

        let mut int_value = number.abs().floor();
        if number.is_sign_negative() {
            int_value = -int_value;
        }

        let mut int8 = int_value % 256.0;
        if int8 > 128.0 {
            int8 -= 256.0;
        }

        int8 as i8
    }

    pub fn to_u8(&self) -> u8 {
        let number = self.to_number().as_f64();

        if number.is_infinite() || number.is_nan() || number == 0.0 {
            return 0;
        }

        let mut int_value = number.abs().floor();
        if number.is_sign_negative() {
            int_value = -int_value;
        }

        (int_value % 256.0) as u8
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
