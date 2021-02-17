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
    fn eval(&mut self, _context: &mut Context) -> Value {
        todo!()
    }
}

#[derive(Debug)]
pub enum Literal {
    Null,
    Boolean(bool),
    Numeric(f64),
    String(String),
}

impl ASTNode for Literal {
    fn eval(&mut self, context: &mut Context) -> Value {
        match self {
            Literal::Null => Value::Null,
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Numeric(v) => Value::Number(*v),
            Literal::String(s) => Value::String(s.clone()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {

    // Numeric operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponent,

    // Bitwise operators
    Or,
    And,
    Xor,
    ShiftLeft,
    ShiftRight,
    UnsignedShiftRight,

    // Logical operators
    BoolOr,
    BoolAnd,

    // Comparision operators
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,

    // Assignment operators
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

    In,
    InstanceOf,
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
}

impl ASTNode for BinaryOperation {
    fn eval(&mut self, context: &mut Context) -> Value {

        // Handle optional evaluation, ex: obj = obj || {}
        match self.op {
            BinaryOp::BoolAnd => {
                let left_value = self.lhs.eval(context);
                if left_value.to_boolean() {
                    return self.rhs.eval(context);
                }

                return left_value;
            },
            BinaryOp::BoolOr => {
                let left_value = self.lhs.eval(context);
                if !left_value.to_boolean() {
                    return self.rhs.eval(context);
                }

                return left_value;
            },
            _ => {},
        }

        let left_value = self.lhs.eval(context);
        let right_value = self.rhs.eval(context);
        if let Value::Number(left_number) = left_value.to_number() {
            if let Value::Number(right_number) = right_value.to_number() {
                return match self.op {
                    BinaryOp::Addition => Value::Number(left_number + right_number),
                    BinaryOp::Subtraction => Value::Number(left_number - right_number),
                    BinaryOp::Multiplication => Value::Number(left_number * right_number),
                    _ => unreachable!(),
                };
            }
        }

        if self.op == BinaryOp::Addition {
            let mut res = left_value.to_string();
            res.push_str(&right_value.to_string());
            return Value::String(res);
        }

        Value::Number(f64::NAN)
    }
}
