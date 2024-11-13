use std::fmt::Display;

use crate::{Example, Solver};

pub struct Part<'a, O>
where
    O: PartialEq + Display,
{
    pub(crate) solver: &'a dyn Solver<O>,
    pub(crate) example: Example<'a, O>,
}

impl<'a, O> Part<'a, O>
where
    O: PartialEq + Display,
{
    pub fn new(solver: &'a dyn Solver<O>, example: Example<'a, O>) -> Self {
        Self { solver, example }
    }

    pub fn get_solution(&self, input: &'a str) -> Result<O, String> {
        self.solver.solve(input)
    }

    pub fn check_example(&self) -> Result<(bool, O), String> {
        // Ok(self.solver.solve(&self.example.input)? == self.example.expected_output)
        let output = self.solver.solve(&self.example.input)?;
        let result = output == self.example.expected_output;
        Ok((result, output))
    }
}
