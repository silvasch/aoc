use std::fmt::Display;

use crate::Part;

pub struct Day<'a, O1, O2>
where
    O1: PartialEq + Display,
    O2: PartialEq + Display,
{
    pub(crate) part_one: Part<'a, O1>,
    pub(crate) part_two: Part<'a, O2>,

    pub(crate) input: &'a str,
}

impl<'a, O1, O2> Day<'a, O1, O2>
where
    O1: PartialEq + Display,
    O2: PartialEq + Display,
{
    pub fn new(part_one: Part<'a, O1>, part_two: Part<'a, O2>, input: &'a str) -> Day<'a, O1, O2> {
        Self {
            part_one,
            part_two,
            input,
        }
    }

    pub fn get_part_one_solution(&self) -> Result<O1, String> {
        self.part_one.get_solution(&self.input)
    }

    pub fn get_part_two_solution(&self) -> Result<O2, String> {
        self.part_two.get_solution(&self.input)
    }

    pub fn check_part_one_example(&self) -> Result<(bool, O1), String> {
        self.part_one.check_example()
    }

    pub fn check_part_two_example(&self) -> Result<(bool, O2), String> {
        self.part_two.check_example()
    }
}
