use super::*;

#[derive(Debug)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    Identifier(Identifier),
    Literal(Literal),
}

impl ASTNode for Expression {
    fn eval(&mut self, context: &mut Context) -> Value {
        match self {
            Expression::BinaryOperation(op) => op.eval(context),
            Expression::Identifier(id) => id.eval(context),
            Expression::Literal(literal) => literal.eval(context),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

impl ASTNode for Identifier {
    fn eval(&mut self, context: &mut Context) -> Value {
        if let Some(v) = context.get_variable(&self.name) {
            v
        } else {
            Value::Undefined
        }
    }
}

impl Identifier {
    pub fn new(str: &str) -> Identifier {
        Identifier {
            name: String::from(str)
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
pub enum Literal {
    Null,
    Boolean(bool),
    Integer(i32),
    Rational(f64),
    BigInt(()),
    String(String),
}

impl Literal {
    pub fn number_from_str(str: &str) -> Literal {
        if let Ok(v) = str.parse::<i32>() {
            Literal::Integer(v)
        } else if let Ok(v) = str.parse::<f64>() {
            Literal::Rational(v)
        } else {
            Literal::Rational(f64::NAN)
        }
    }
}

impl ASTNode for Literal {
    fn eval(&mut self, context: &mut Context) -> Value {
        match self {
            Literal::Null => Value::Null,
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Integer(v) => Value::Integer(*v),
            Literal::Rational(v) => Value::Rational(*v),
            Literal::BigInt(_) => todo!(),
            Literal::String(s) => Value::String(s.clone()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BitwiseOp {
    Or,
    And,
    Xor,
    ShiftLeft,
    ShiftRight,
    UnsignedShiftRight,
}


#[derive(Debug, PartialEq)]
pub enum AssignmentOp {
    AdditionAssignment,
    SubtractionAssignment,
    MultiplicationAssignment,
    DivisionAssignment,
    ModuloAssignment,
    ExponentAssignment,

    BitAndAssignment,
    BitOrAssignment,
    BitXorAssignment,

    ShiftLeftAssignment,
    ShiftRightAssignment,
    UnsignedShiftRightAssignment,

    BoolAndAssignment,
    BoolOrAssignment,
}

#[derive(Debug, PartialEq)]
pub enum NumericOp {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponent,
}

#[derive(Debug, PartialEq)]
pub enum CompareOp {
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    In,
    InstanceOf,
}

#[derive(Debug)]
pub enum BinaryOp {
    // Logical operators
    BoolOr,
    BoolAnd,

    NumericOp(NumericOp),
    CompareOp(CompareOp),
    BitwiseOp(BitwiseOp),
    AssignmentOp(AssignmentOp),
}

#[derive(Debug)]
pub struct BinaryOperation {
    op: BinaryOp,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl BinaryOperation {
    pub fn new(op: BinaryOp, lhs: Expression, rhs: Expression) -> BinaryOperation {
        BinaryOperation {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn numeric(op: NumericOp, lhs: Expression, rhs: Expression) -> BinaryOperation {
        Self::new(BinaryOp::NumericOp(op), lhs, rhs)
    }

    pub fn compare(op: CompareOp, lhs: Expression, rhs: Expression) -> BinaryOperation {
        Self::new(BinaryOp::CompareOp(op), lhs, rhs)
    }

    pub fn bitwise(op: BitwiseOp, lhs: Expression, rhs: Expression) -> BinaryOperation {
        Self::new(BinaryOp::BitwiseOp(op), lhs, rhs)
    }

    pub fn assignment(op: AssignmentOp, lhs: Expression, rhs: Expression) -> BinaryOperation {
        Self::new(BinaryOp::AssignmentOp(op), lhs, rhs)
    }
}

impl BinaryOperation {
    fn do_numeric_op(&mut self, context: &mut Context) -> Value {
        let op = if let BinaryOp::NumericOp(op) = &self.op {
           op
        } else {
            panic!("Called do_numeric_op on non-numeric operation");
        };

        let left_value = self.lhs.eval(context).to_numeric();
        let right_value = self.rhs.eval(context).to_numeric();

        if let Value::Integer(left) = left_value {
            if let Value::Integer(right) = right_value {
                return match op {
                    NumericOp::Addition => Value::Integer(left + right),
                    NumericOp::Subtraction => Value::Integer(left - right),
                    NumericOp::Multiplication => Value::Integer(left * right),
                    NumericOp::Modulo => Value::Integer(left % right),
                    NumericOp::Division => {
                        let result = left / right;
                        if result * right == left {
                            Value::Integer(result)
                        } else {
                            Value::Rational((left as f64) / (right as f64))
                        }
                    },
                    NumericOp::Exponent => {
                        if right >= 0 {
                            Value::Integer(left.pow(right as u32))
                        } else {
                            Value::Rational((left as f64).powi(right))
                        }
                    }
                };
            }
        }

        match (&left_value, &right_value) {
            (Value::Integer(_), Value::Rational(_))
            | (Value::Rational(_), Value::Integer(_))
            | (Value::Rational(_), Value::Rational(_))
            => {
                let left = left_value.as_f64();
                let right = right_value.as_f64();

                return match op {
                    NumericOp::Addition => Value::Rational(left + right),
                    NumericOp::Subtraction => Value::Rational(left - right),
                    NumericOp::Multiplication => Value::Rational(left * right),
                    NumericOp::Modulo => Value::Rational(left % right),
                    NumericOp::Division => Value::Rational(left / right),
                    NumericOp::Exponent => Value::Rational(left.powf(right)),
                };
            }
            _ => {}
        }

        if op == &NumericOp::Addition {
            let mut res = left_value.to_string();
            res.push_str(&right_value.to_string());
            return Value::String(res);
        }

        Value::nan()
    }

    fn do_compare_op(&mut self, context: &mut Context) -> Value {
        todo!()
    }

    fn do_bitwise_op(&mut self, context: &mut Context) -> Value {
        if let BinaryOp::BitwiseOp(op) = &self.op {
            let left_value = self.lhs.eval(context);
            let right_value = self.rhs.eval(context);
            let left = left_value.to_i32();
            let right = right_value.to_i32();

            return match op {
                BitwiseOp::Or => Value::Integer(left | right),
                BitwiseOp::And => Value::Integer(left & right),
                BitwiseOp::Xor => Value::Integer(left ^ right),
                BitwiseOp::ShiftLeft => Value::Integer(left << (right % 32)),
                BitwiseOp::ShiftRight => Value::Integer(left >> (right % 32)),
                BitwiseOp::UnsignedShiftRight => Value::Integer(
                    (left_value.to_u32() >> (right_value.to_u32() % 32)) as i32
                ),
            }
        }

        unreachable!()
    }

    fn do_assignment_op(&mut self, context: &mut Context) -> Value {
        todo!()
    }
}

impl ASTNode for BinaryOperation {
    fn eval(&mut self, context: &mut Context) -> Value {
        match self.op {
            BinaryOp::NumericOp(_) => self.do_numeric_op(context),
            BinaryOp::CompareOp(_) => self.do_compare_op(context),
            BinaryOp::BitwiseOp(_) => self.do_bitwise_op(context),
            BinaryOp::AssignmentOp(_) => self.do_assignment_op(context),

            BinaryOp::BoolAnd => {
                let left_value = self.lhs.eval(context);
                if left_value.to_boolean() {
                    self.rhs.eval(context)
                } else {
                    left_value
                }
            },
            BinaryOp::BoolOr => {
                let left_value = self.lhs.eval(context);
                if !left_value.to_boolean() {
                    self.rhs.eval(context)
                } else {
                    left_value
                }
            },
        }
    }
}