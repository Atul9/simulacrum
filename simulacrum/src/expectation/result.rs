use std::fmt;
use constraint::{ConstraintError, ConstraintResult};

use MethodName;

pub type ExpectationResult = Result<(), ExpectationError>;

#[derive(Debug, PartialEq)]
pub struct ExpectationError {
    pub constraint_err: ConstraintError,
    pub method_name: MethodName
}

impl fmt::Display for ExpectationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.method_name, self.constraint_err)
    }
}