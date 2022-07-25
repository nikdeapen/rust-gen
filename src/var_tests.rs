use code_gen::CodeBuffer;

use crate::{TypeTag, Var};

#[test]
fn display() {
    let var: Var = Var::new("name", TypeTag::from("type"));
    let result: String = CodeBuffer::display_expression(&var);
    assert_eq!(result.as_str(), "name: type");
}
