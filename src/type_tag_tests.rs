use code_gen::CodeBuffer;

use crate::TypeTag;

#[test]
fn display() {
    let tag: TypeTag = TypeTag::from("name");
    let result: String = CodeBuffer::display_expression(&tag);
    assert_eq!(result.as_str(), "name");
}
