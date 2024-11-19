use std::fmt::Display;

use crate::Part;

pub struct Day<'a, O1, O2>
where
    O1: Display + PartialEq,
    O2: Display + PartialEq,
{
    day_number: u8,
    day_name: &'a str,

    input: &'a str,

    part_one: Part<'a, O1>,
    part_two: Part<'a, O2>,
}

impl<'a, O1, O2> Day<'a, O1, O2>
where
    O1: Display + PartialEq,
    O2: Display + PartialEq,
{
    pub fn new(
        day_number: u8,
        day_name: &'a str,
        input: &'a str,
        part_one: Part<'a, O1>,
        part_two: Part<'a, O2>,
    ) -> Self {
        Self {
            day_number,
            day_name,
            input,
            part_one,
            part_two,
        }
    }

    pub(crate) fn solve_one(&self) -> Result<O1, String> {
        self.part_one.solve(self.input)
    }

    pub(crate) fn solve_two(&self) -> Result<O2, String> {
        self.part_two.solve(self.input)
    }

    pub(crate) fn check_one(&self) -> Result<(bool, O1), String> {
        self.part_one.check()
    }

    pub(crate) fn check_two(&self) -> Result<(bool, O2), String> {
        self.part_two.check()
    }

    pub(crate) fn get_solver_one_name(&self) -> &'a str {
        self.part_one.get_solver_name()
    }

    pub(crate) fn get_solver_two_name(&self) -> &'a str {
        self.part_two.get_solver_name()
    }

    pub(crate) fn get_day_description(&self) -> String {
        format!(
            "Advent of Code - Day {}: {}",
            self.day_number, self.day_name
        )
    }
}
