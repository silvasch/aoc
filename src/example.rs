use std::fmt::Display;

pub struct Example<'a, O>
where
    O: Display + PartialEq,
{
    pub(crate) input: &'a str,
    pub(crate) expected_output: O,
}

impl<'a, O> Example<'a, O>
where
    O: Display + PartialEq,
{
    pub fn new(input: &'a str, expected_output: O) -> Self {
        Self {
            input,
            expected_output,
        }
    }
}
