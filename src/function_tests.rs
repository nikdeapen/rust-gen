use code_gen::{CodeBuffer, Literal};

use crate::{Function, SemiExpression, TypeTag, Var};

#[test]
fn display() {
    let tag: TypeTag = TypeTag::from("TheType");
    let function: Function = Function::from("fun1")
        .with_parameter(Var::new("a", tag.clone()))
        .with_parameter(Var::new("b", tag.clone()))
        .with_result(tag)
        .with_statement(SemiExpression::from(Literal::from("one")))
        .with_statement(SemiExpression::from(Literal::from("two")));
    let result: String = CodeBuffer::display_statement(&function);
    assert_eq!(
        result,
        "fn fun1(a: TheType, b: TheType) -> TheType {\n\tone;\n\ttwo;\n}\n"
    );

    let function: Function = Function::from("fun1");
    let result: String = CodeBuffer::display_statement(&function);
    assert_eq!(result, "fn fun1() {}\n");
}
