use std::fmt::Display;

use crate::{Example, Solver};

pub struct Part<'a, O>
where
    O: Display + PartialEq,
{
    solver: &'a dyn Solver<'a, O>,
    example: Example<'a, O>,
}

impl<'a, O> Part<'a, O>
where
    O: Display + PartialEq,
{
    pub fn new(solver: &'a dyn Solver<'a, O>, example: Example<'a, O>) -> Self {
        Self { solver, example }
    }

    pub(crate) fn solve(&self, input: &'a str) -> Result<O, String> {
        match self.solver.solve(input) {
            Some(output) => output,
            None => Err(format!("'{}' is not yet implemented.", self.solver.name())),
        }
    }

    pub(crate) fn check(&self) -> Result<(bool, O), String> {
        let output = self.solve(self.example.input)?;
        Ok((output == self.example.expected_output, output))
    }

    pub(crate) fn get_solver_name(&self) -> &'a str {
        self.solver.name()
    }
}
