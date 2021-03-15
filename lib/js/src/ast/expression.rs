use super::*;

#[derive(Debug)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    Identifier(Identifier),
    Literal(Literal),
}

impl ASTNode for Expression {
    fn eval(&mut self, context: &mut Context) -> Result<Value> {
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
    fn eval(&mut self, context: &mut Context) -> Result<Value> {
        if let Some(v) = context.get_variable(&self.name) {
            Ok(v)
        } else {
            Ok(Value::Undefined)
        }
    }
}

impl Identifier {
    pub fn new(str: &str) -> Identifier {
        Identifier {
            name: String::from(str),
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
    fn eval(&mut self, _context: &mut Context) -> Result<Value> {
        Ok(match self {
            Literal::Null => Value::Null,
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Integer(v) => Value::Integer(*v),
            Literal::Rational(v) => Value::Rational(*v),
            Literal::BigInt(_) => todo!(),
            Literal::String(s) => Value::String(s.clone()),
        })
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
    Assignment,

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
    fn do_numeric_op(op: &NumericOp, left_value: Value, right_value: Value) -> Value {
        let left_value = left_value.to_number();
        let right_value = right_value.to_number();

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
                    }
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
            | (Value::Rational(_), Value::Rational(_)) => {
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

    fn do_bitwise_op(op: &BitwiseOp, left_value: Value, right_value: Value) -> Value {
        let left_value = left_value.to_number();
        let right_value = right_value.to_number();
        let left = left_value.to_i32();
        let right = right_value.to_i32();

        match op {
            BitwiseOp::Or => Value::Integer(left | right),
            BitwiseOp::And => Value::Integer(left & right),
            BitwiseOp::Xor => Value::Integer(left ^ right),
            BitwiseOp::ShiftLeft => Value::Integer(left << (right % 32)),
            BitwiseOp::ShiftRight => Value::Integer(left >> (right % 32)),
            BitwiseOp::UnsignedShiftRight => {
                Value::Integer((left_value.to_u32() >> (right_value.to_u32() % 32)) as i32)
            }
        }
    }

    fn do_compare_op(op: &CompareOp, left_value: Value, right_value: Value) -> Result<Value> {
        todo!()
    }

    fn do_assignment_op(&mut self, context: &mut Context) -> Result<Value> {
        let op = if let BinaryOp::AssignmentOp(op) = &self.op {
            op
        } else {
            panic!("Called do_assignment_op on non-assignment operation");
        };

        let left_value = self.lhs.eval(context)?;
        let right_value = self.rhs.eval(context)?;
        if let Expression::Identifier(ident) = self.lhs.as_ref() {
            let value = match op {
                AssignmentOp::Assignment => right_value,
                AssignmentOp::AdditionAssignment => {
                    Self::do_numeric_op(&NumericOp::Addition, left_value, right_value)
                }
                AssignmentOp::SubtractionAssignment => {
                    Self::do_numeric_op(&NumericOp::Subtraction, left_value, right_value)
                }
                AssignmentOp::MultiplicationAssignment => {
                    Self::do_numeric_op(&NumericOp::Multiplication, left_value, right_value)
                }
                AssignmentOp::DivisionAssignment => {
                    Self::do_numeric_op(&NumericOp::Division, left_value, right_value)
                }
                AssignmentOp::ModuloAssignment => {
                    Self::do_numeric_op(&NumericOp::Modulo, left_value, right_value)
                }
                AssignmentOp::ExponentAssignment => {
                    Self::do_numeric_op(&NumericOp::Exponent, left_value, right_value)
                }
                AssignmentOp::BitAndAssignment => {
                    Self::do_bitwise_op(&BitwiseOp::And, left_value, right_value)
                }
                AssignmentOp::BitOrAssignment => {
                    Self::do_bitwise_op(&BitwiseOp::Or, left_value, right_value)
                }
                AssignmentOp::BitXorAssignment => {
                    Self::do_bitwise_op(&BitwiseOp::Xor, left_value, right_value)
                }
                AssignmentOp::ShiftLeftAssignment => {
                    Self::do_bitwise_op(&BitwiseOp::ShiftLeft, left_value, right_value)
                }
                AssignmentOp::ShiftRightAssignment => {
                    Self::do_bitwise_op(&BitwiseOp::ShiftRight, left_value, right_value)
                }
                AssignmentOp::UnsignedShiftRightAssignment => {
                    Self::do_bitwise_op(&BitwiseOp::UnsignedShiftRight, left_value, right_value)
                }
                AssignmentOp::BoolAndAssignment => {
                    Value::Boolean(left_value.to_boolean() && right_value.to_boolean())
                }
                AssignmentOp::BoolOrAssignment => {
                    Value::Boolean(left_value.to_boolean() || right_value.to_boolean())
                }
            };

            context.set_variable(ident.name().clone(), value.clone());
            Ok(value)
        } else {
            unreachable!()
        }
    }
}

impl ASTNode for BinaryOperation {
    fn eval(&mut self, context: &mut Context) -> Result<Value> {
        Ok(match &self.op {
            BinaryOp::NumericOp(_) | BinaryOp::BitwiseOp(_) | BinaryOp::CompareOp(_) => {
                let left_value = self.lhs.eval(context)?;
                let right_value = self.rhs.eval(context)?;

                match &self.op {
                    BinaryOp::NumericOp(op) => Self::do_numeric_op(&op, left_value, right_value),
                    BinaryOp::BitwiseOp(op) => Self::do_bitwise_op(&op, left_value, right_value),
                    BinaryOp::CompareOp(op) => Self::do_compare_op(&op, left_value, right_value)?,
                    _ => unreachable!(),
                }
            }
            BinaryOp::AssignmentOp(_) => self.do_assignment_op(context)?,
            BinaryOp::BoolAnd => {
                let left_value = self.lhs.eval(context)?;
                if left_value.to_boolean() {
                    self.rhs.eval(context)?
                } else {
                    left_value
                }
            }
            BinaryOp::BoolOr => {
                let left_value = self.lhs.eval(context)?;
                if !left_value.to_boolean() {
                    self.rhs.eval(context)?
                } else {
                    left_value
                }
            }
        })
    }
}
