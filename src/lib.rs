use std::collections::HashMap;

pub struct Context;

pub trait Expression {
    fn eval(&mut self, context: &mut Context) -> Value;
}

#[derive(Debug, Clone)]
pub enum Value {
    Undefined,
    Null,
    Number(f64),
    String(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Function(),
}

impl Expression for Value {
    fn eval(&mut self, _: &mut Context) -> Value {
        self.clone()
    }
}

impl Value {
    pub fn to_str(&self) -> String {
        match self {
            Value::Undefined => "undefined".to_owned(),
            Value::Null => "null".to_owned(),
            Value::Number(v) => v.to_string(),
            Value::String(v) => v.clone(),
            Value::Object(_) => "[object".to_owned(),
            Value::Array(_) => "[array]".to_owned(),
            Value::Function() => "[function]".to_owned()
        }
    }

    pub fn to_num(&self) -> Option<f64> {
        match self {
            Value::Number(v) => Some(*v),
            Value::String(s) => {
                if let Ok(v) = s.parse::<f64>() {
                    Some(v)
                } else {
                    None
                }            
            },
            _ => None
        }
    }
}

pub enum Operator {
    Invalid,

    Add  {lhs: Box<dyn Expression>, rhs: Box<dyn Expression>},
    Sub  {lhs: Box<dyn Expression>, rhs: Box<dyn Expression>},
    Mult {lhs: Box<dyn Expression>, rhs: Box<dyn Expression>},
}

impl Expression for Operator {
    fn eval(&mut self, context: &mut Context) -> Value {
        match self {
            Operator::Invalid => Value::Undefined,
            Operator::Add { lhs, rhs } => {  
                let left_value = lhs.eval(context);
                let right_value = rhs.eval(context);
                if let Some(left_number) = left_value.to_num() {                     
                    if let Some(right_number) = right_value.to_num() {
                        return Value::Number(left_number + right_number)
                    }
                }

                let mut res = left_value.to_str();
                res.push_str(&right_value.to_str());

                Value::String(res)
            },
            Operator::Sub { lhs, rhs } => {  
                let left_value = lhs.eval(context);
                let right_value = rhs.eval(context);
                if let Some(left_number) = left_value.to_num() {                     
                    if let Some(right_number) = right_value.to_num() {
                        return Value::Number(left_number - right_number)
                    }
                }

                Value::Number(f64::NAN)
            },
            Operator::Mult { lhs, rhs } => {  
                let left_value = lhs.eval(context);
                let right_value = rhs.eval(context);
                if let Some(left_number) = left_value.to_num() {                     
                    if let Some(right_number) = right_value.to_num() {
                        return Value::Number(left_number * right_number)
                    }
                }

                Value::Number(f64::NAN)
            }
        }
    }
}

