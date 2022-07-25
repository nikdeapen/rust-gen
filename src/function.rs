use code_gen::{CodeBuffer, Expression, Statement};

use crate::{TypeTag, Var};

/// A function declaration.
pub struct Function {
    name: String,
    result: Option<TypeTag>,
    parameters: Vec<Var>,
    statements: Vec<Box<dyn Statement>>,
}

impl From<String> for Function {
    fn from(name: String) -> Self {
        Self {
            name,
            result: None,
            parameters: Vec::new(),
            statements: Vec::new(),
        }
    }
}

impl From<&str> for Function {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

impl Function {
    //! Mutations

    /// Sets the result.
    pub fn with_result(mut self, result: TypeTag) -> Self {
        self.result = Some(result);
        self
    }

    /// Adds the parameter.
    pub fn with_parameter(mut self, parameter: Var) -> Self {
        self.parameters.push(parameter);
        self
    }

    /// Adds the statement.
    pub fn with_statement<S: 'static + Statement>(mut self, statement: S) -> Self {
        self.statements.push(Box::new(statement));
        self
    }
}

impl Statement for Function {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("fn ");
        b.write(self.name.as_str());
        b.write("(");
        for (i, parameter) in self.parameters.iter().enumerate() {
            if i != 0 {
                b.write(", ");
            }
            parameter.write(b);
        }
        b.write(")");
        match &self.result {
            Some(result) => {
                b.write(" -> ");
                result.write(b);
            }
            None => {}
        }
        b.write(" {");
        if self.statements.is_empty() {
            b.write("}");
            b.end_line();
            return;
        }
        b.end_line();

        for statement in &self.statements {
            statement.write(b, level + 1);
        }

        b.line(level, "}");
    }
}
