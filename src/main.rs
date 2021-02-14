use rust_js::{ast::Expression, ast::Operator, js::Value, Context};

fn main() {
    let mut context = Context {};

    let mut expr = Operator::Add {
        lhs: Box::new(Operator::Mult {
            lhs: Box::new(Value::Number(4.0)),
            rhs: Box::new(Value::Number(3.0)),
        }),
        rhs: Box::new(Operator::Sub {
            lhs: Box::new(Value::Number(2.0)),
            rhs: Box::new(Value::Number(1.0)),
        }),
    };

    let value = expr.eval(&mut context);

    println!("Exec value: {:?}", value);
}
