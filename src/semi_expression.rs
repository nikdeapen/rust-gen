use code_gen::{CodeBuffer, Expression, Statement};

/// A semi-colon ended expression statement.
pub struct SemiExpression<E: Expression> {
    expression: E,
}

impl<E: Expression> From<E> for SemiExpression<E> {
    fn from(expression: E) -> Self {
        Self { expression }
    }
}

impl<E: Expression> Statement for SemiExpression<E> {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.expression.write(b);
        b.write(";");
        b.end_line();
    }
}
