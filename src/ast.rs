use crate::js::*;
use crate::*;

pub trait Expression {
    fn eval(&mut self, context: &mut Context) -> Value;
}

pub enum Operator {
    Invalid,

    Add { lhs: Box<dyn Expression>, rhs: Box<dyn Expression> },
    Sub { lhs: Box<dyn Expression>, rhs: Box<dyn Expression> },
    Mult { lhs: Box<dyn Expression>, rhs: Box<dyn Expression> },
}

impl Expression for Operator {
    fn eval(&mut self, context: &mut Context) -> Value {
        match self {
            Operator::Invalid => Value::Undefined,
            Operator::Add { lhs, rhs } => {
                let left_value = lhs.eval(context);
                let right_value = rhs.eval(context);
                if let Value::Number(left_number) = left_value.to_number() {
                    if let Value::Number(right_number) = right_value.to_number() {
                        return Value::Number(left_number + right_number);
                    }
                }

                let mut res = left_value.to_string();
                res.push_str(&right_value.to_string());

                Value::String(res)
            }
            Operator::Sub { lhs, rhs } => {
                let left_value = lhs.eval(context);
                let right_value = rhs.eval(context);
                if let Value::Number(left_number) = left_value.to_number() {
                    if let Value::Number(right_number) = right_value.to_number() {
                        return Value::Number(left_number - right_number);
                    }
                }

                Value::Number(f64::NAN)
            }
            Operator::Mult { lhs, rhs } => {
                let left_value = lhs.eval(context);
                let right_value = rhs.eval(context);
                if let Value::Number(left_number) = left_value.to_number() {
                    if let Value::Number(right_number) = right_value.to_number() {
                        return Value::Number(left_number * right_number);
                    }
                }

                Value::Number(f64::NAN)
            }
        }
    }
}
