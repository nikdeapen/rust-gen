use code_gen::{CodeBuffer, Expression};

/// A type.
#[derive(Clone, Debug)]
pub enum TypeTag {
    Named(String),
}

impl From<String> for TypeTag {
    fn from(name: String) -> Self {
        Self::Named(name)
    }
}

impl From<&str> for TypeTag {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

impl Expression for TypeTag {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Named(name) => b.write(name.as_str()),
        }
    }
}
