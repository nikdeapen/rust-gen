use code_gen::{CodeBuffer, Literal};

use crate::SemiExpression;

#[test]
fn display() {
    let semi: SemiExpression<Literal> = SemiExpression::from(Literal::from("value"));
    let result: String = CodeBuffer::display_statement(&semi);
    assert_eq!(result.as_str(), "value;\n");
}
