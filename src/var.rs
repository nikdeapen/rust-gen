use code_gen::{CodeBuffer, Expression};

use crate::TypeTag;

/// A named type.
#[derive(Clone, Debug)]
pub struct Var {
    name: String,
    tag: TypeTag,
}

impl Var {
    //! Constructors

    /// Creates a new var.
    pub fn new(name: &str, tag: TypeTag) -> Self {
        Self {
            name: name.to_string(),
            tag,
        }
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        b.write(self.name.as_str());
        b.write(": ");
        self.tag.write(b);
    }
}
