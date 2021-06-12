use super::*;

#[derive(Debug)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    Identifier(Identifier),
    Literal(Literal),
    CallExpression(CallExpression),
    ObjectExpression(ObjectExpression),
}

impl ASTNode for Expression {
    fn eval(&mut self, context: &mut Context) -> Result<JsValue> {
        match self {
            Expression::BinaryOperation(e) => e.eval(context),
            Expression::Identifier(e) => e.eval(context),
            Expression::Literal(e) => e.eval(context),
            Expression::CallExpression(e) => e.eval(context),
            Expression::ObjectExpression(e) => e.eval(context),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

impl ASTNode for Identifier {
    fn eval(&mut self, context: &mut Context) -> Result<JsValue> {
        Ok(context.get_variable(&self.name))
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
    fn eval(&mut self, context: &mut Context) -> Result<JsValue> {
        Ok(match self {
            Literal::Null => JsValue::null(),
            Literal::Boolean(b) => JsValue::bool(*b),
            Literal::Integer(v) => JsValue::integer(*v),
            Literal::Rational(v) => JsValue::rational(*v),
            Literal::BigInt(_) => todo!(),
            Literal::String(s) => JsValue::string(s.as_ref()),
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
    pub fn do_numeric_op(op: &NumericOp, left_value: JsValue, right_value: JsValue) -> JsValue {
        let left_value = left_value.to_number();
        let right_value = right_value.to_number();

        if let JsValue::Integer(left) = left_value {
            if let JsValue::Integer(right) = right_value {
                return match op {
                    NumericOp::Addition => JsValue::Integer(left + right),
                    NumericOp::Subtraction => JsValue::Integer(left - right),
                    NumericOp::Multiplication => JsValue::Integer(left * right),
                    NumericOp::Modulo => JsValue::Integer(left % right),
                    NumericOp::Division => {
                        let result = left / right;
                        if result * right == left {
                            JsValue::Integer(result)
                        } else {
                            JsValue::Rational((left as f64) / (right as f64))
                        }
                    }
                    NumericOp::Exponent => {
                        if right >= 0 {
                            JsValue::Integer(left.pow(right as u32))
                        } else {
                            JsValue::Rational((left as f64).powi(right))
                        }
                    }
                };
            }
        }

        match (&left_value, &right_value) {
            (JsValue::Integer(_), JsValue::Rational(_))
            | (JsValue::Rational(_), JsValue::Integer(_))
            | (JsValue::Rational(_), JsValue::Rational(_)) => {
                let left = left_value.as_f64();
                let right = right_value.as_f64();

                return match op {
                    NumericOp::Addition => JsValue::Rational(left + right),
                    NumericOp::Subtraction => JsValue::Rational(left - right),
                    NumericOp::Multiplication => JsValue::Rational(left * right),
                    NumericOp::Modulo => JsValue::Rational(left % right),
                    NumericOp::Division => JsValue::Rational(left / right),
                    NumericOp::Exponent => JsValue::Rational(left.powf(right)),
                };
            }
            _ => {}
        }

        if op == &NumericOp::Addition {
            let mut res = left_value.to_string();
            res.push_str(&right_value.to_string());
            return JsValue::string(res.as_ref());
        }

        JsValue::nan()
    }

    pub fn do_bitwise_op(op: &BitwiseOp, left_value: JsValue, right_value: JsValue) -> JsValue {
        let left_value = left_value.to_number();
        let right_value = right_value.to_number();
        let left = left_value.to_i32();
        let right = right_value.to_i32();

        match op {
            BitwiseOp::Or => JsValue::Integer(left | right),
            BitwiseOp::And => JsValue::Integer(left & right),
            BitwiseOp::Xor => JsValue::Integer(left ^ right),
            BitwiseOp::ShiftLeft => JsValue::Integer(left << (right % 32)),
            BitwiseOp::ShiftRight => JsValue::Integer(left >> (right % 32)),
            BitwiseOp::UnsignedShiftRight => {
                JsValue::Integer((left_value.to_u32() >> (right_value.to_u32() % 32)) as i32)
            }
        }
    }

    pub fn do_compare_op(
        op: &CompareOp,
        left_value: JsValue,
        right_value: JsValue,
    ) -> Result<JsValue> {
        todo!()
    }

    pub fn do_assignment_op(&mut self, context: &mut Context) -> Result<JsValue> {
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
                    JsValue::Boolean(left_value.to_boolean() && right_value.to_boolean())
                }
                AssignmentOp::BoolOrAssignment => {
                    JsValue::Boolean(left_value.to_boolean() || right_value.to_boolean())
                }
            };

            let gc_value = context.allocate(value.clone());
            context.set_variable(ident.name().clone(), gc_value);
            Ok(value)
        } else {
            unreachable!()
        }
    }
}

impl ASTNode for BinaryOperation {
    fn eval(&mut self, context: &mut Context) -> Result<JsValue> {
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

#[derive(Debug)]
pub struct ObjectExpression(Vec<ObjectProperty>);

impl ObjectExpression {
    pub fn new(properties: Vec<ObjectProperty>) -> Self {
        Self(properties)
    }
}

impl ASTNode for ObjectExpression {
    fn eval(&mut self, context: &mut Context) -> Result<JsValue> {
        let mut obj = JsObject::new(context);
        for prop in self.0.iter_mut() {
            let key = if let Expression::Identifier(ident) = prop.key.as_mut() {
                JsValue::string(ident.name().as_ref())
            } else {
                prop.key.eval(context)?
            };

            let value = prop.value.as_mut().unwrap().eval(context)?;
            let value = context.allocate(value);
            obj.put_property(key.to_primitive_string(context), value);
        }

        Ok(JsValue::object(obj))
    }
}

#[derive(Debug)]
pub enum ObjectPropertyKind {
    KeyValue,
    Getter,
    Setter,
    Spread,
}

#[derive(Debug)]
pub struct ObjectProperty {
    key: Box<Expression>,
    value: Option<Box<Expression>>,
    kind: ObjectPropertyKind,
    is_method: bool,
}

impl ObjectProperty {
    pub fn new(
        key: Expression,
        value: Option<Expression>,
        kind: ObjectPropertyKind,
        is_method: bool,
    ) -> ObjectProperty {
        ObjectProperty {
            key: Box::new(key),
            value: value.map(Box::new),
            kind,
            is_method,
        }
    }
}

impl ASTNode for ObjectProperty {
    fn eval(&mut self, _context: &mut Context) -> Result<JsValue> {
        unreachable!()
    }
}

#[derive(Debug)]
pub struct CallExpression {
    ident: Box<Expression>,
    args: Vec<Expression>,
}

impl CallExpression {
    pub fn new(ident: Expression, args: Vec<Expression>) -> CallExpression {
        CallExpression {
            ident: Box::new(ident),
            args,
        }
    }
}

impl ASTNode for CallExpression {
    fn eval(&mut self, context: &mut Context) -> Result<JsValue> {
        todo!()
    }
}
