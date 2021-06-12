use std::{fmt, ops::Deref};

use utils::prelude::FlyString;

use super::{string::JsString, JsObject};
use crate::{gc::*, vm::Context};

#[derive(Clone, GcTrace)]
pub enum JsValue {
    Undefined,
    Null,
    Boolean(bool),
    Rational(f64),
    Integer(i32),
    BigInt(()),
    String(JsString),
    Object(GcPointer<JsObject>),
    Symbol(()),
}

impl GcCell for JsValue {}

pub enum PreferredType {
    None,
    Number,
    String,
}

impl fmt::Debug for JsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsValue::Undefined => write!(f, "Value{{undefined}}"),
            JsValue::Null => write!(f, "Value{{null}}"),
            JsValue::Boolean(b) => write!(f, "Value{{bool: {}}}", b),
            JsValue::Rational(v) => write!(f, "Value{{number: {}}}", v),
            JsValue::Integer(v) => write!(f, "Value{{number: {}}}", v),
            JsValue::String(s) => write!(f, "Value{{string: {}}}", s.clone().string),
            JsValue::Object(o) => write!(f, "Value{{object: {:#?}}}", o),
            JsValue::BigInt(_) => write!(f, "Value{{bigint}}"),
            JsValue::Symbol(_) => write!(f, "Value{{symbol}}"),
        }
    }
}

impl JsValue {
    // Constructors

    pub fn nan() -> JsValue {
        JsValue::Rational(f64::NAN)
    }

    pub fn undefined() -> JsValue {
        JsValue::Undefined
    }

    pub fn null() -> JsValue {
        JsValue::Null
    }

    pub fn bool(b: bool) -> JsValue {
        JsValue::Boolean(b)
    }

    pub fn integer(v: i32) -> JsValue {
        JsValue::Integer(v)
    }

    pub fn rational(v: f64) -> JsValue {
        JsValue::Rational(v)
    }

    pub fn string<T: Into<FlyString>>(str: T) -> JsValue {
        JsValue::String(JsString::new(str.into()))
    }

    pub fn object(obj: GcPointer<JsObject>) -> JsValue {
        JsValue::Object(obj)
    }

    pub fn number_from_str(str: &str) -> JsValue {
        if let Ok(v) = str.parse::<i32>() {
            JsValue::Integer(v)
        } else if let Ok(v) = str.parse::<f64>() {
            JsValue::Rational(v)
        } else {
            JsValue::nan()
        }
    }

    // Utility

    pub fn as_f64(&self) -> f64 {
        match self {
            JsValue::Rational(v) => *v,
            JsValue::Integer(v) => *v as f64,
            _ => panic!("Failed to cast {:?} as f64", self),
        }
    }

    pub fn as_i32(&self) -> i32 {
        match self {
            JsValue::Rational(v) => *v as i32,
            JsValue::Integer(v) => *v as i32,
            _ => panic!("Failed to cast {:?} as T", self),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, JsValue::Null)
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, JsValue::Undefined)
    }

    pub fn is_bigint(&self) -> bool {
        matches!(self, JsValue::BigInt(_))
    }

    pub fn is_nan(&self) -> bool {
        if let JsValue::Rational(v) = self {
            v.is_nan()
        } else {
            false
        }
    }

    pub fn is_infinity(&self) -> bool {
        if let JsValue::Rational(v) = self {
            v.is_infinite()
        } else {
            false
        }
    }

    // JS Value type conversion

    pub fn to_primitive(&self, _preferred_type: PreferredType) -> JsValue {
        if let JsValue::Object(obj) = self {
            JsValue::string(format!("Object: {:?}", obj).as_ref())
        } else {
            self.clone()
        }
    }

    pub fn to_boolean(&self) -> bool {
        match &self {
            JsValue::Undefined => false,
            JsValue::Null => false,
            JsValue::Boolean(v) => *v,
            JsValue::Rational(v) => !(v.is_nan() && *v == 0.0),
            JsValue::Integer(v) => *v == 0,
            JsValue::BigInt(_) => todo!(),
            JsValue::Symbol(_) => true,
            JsValue::String(s) => !s.string.is_empty(),
            JsValue::Object(_) => true,
        }
    }

    pub fn to_number(&self) -> JsValue {
        match self {
            JsValue::Undefined => JsValue::nan(),
            JsValue::Null => JsValue::Integer(0),
            JsValue::Boolean(b) => JsValue::Integer(if *b { 1 } else { 0 }),
            JsValue::Rational(_) => self.clone(),
            JsValue::Integer(_) => self.clone(),
            JsValue::BigInt(_) => panic!("TypeError"),
            JsValue::Symbol(_) => panic!("TypeError"),
            JsValue::String(s) => {
                JsValue::Rational(*s.string.parse::<f64>().ok().get_or_insert(f64::NAN))
            }
            JsValue::Object(_) => self.to_primitive(PreferredType::Number).to_number(),
        }
    }

    pub fn to_numeric(&self) -> JsValue {
        let primitive_value = self.to_primitive(PreferredType::Number);
        if primitive_value.is_bigint() {
            primitive_value
        } else {
            primitive_value.to_number()
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            JsValue::Undefined => "undefined".into(),
            JsValue::Null => "null".into(),
            JsValue::Boolean(v) => v.to_string(),
            JsValue::Rational(v) => v.to_string(),
            JsValue::Integer(v) => v.to_string(),
            JsValue::BigInt(_) => todo!(),
            JsValue::String(v) => v.clone().string.to_string(),
            JsValue::Object(_) => self.to_primitive(PreferredType::String).to_string(),
            JsValue::Symbol(_) => panic!("TypeError"),
        }
    }

    pub fn to_primitive_string(&self, context: &mut Context) -> GcPointer<JsString> {
        context.allocate(JsString::new(self.to_string().as_ref()))
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

impl From<f64> for JsValue {
    fn from(v: f64) -> Self {
        JsValue::Rational(v)
    }
}

impl From<i32> for JsValue {
    fn from(v: i32) -> Self {
        JsValue::Integer(v)
    }
}
