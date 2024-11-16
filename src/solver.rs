use std::fmt::Display;

pub trait Solver<'a, O>
where
    O: Display + PartialEq,
{
    #[allow(unused)]
    fn solve(&self, input: &str) -> Option<Result<O, String>> {
        None
    }
    fn name(&self) -> &'a str;
}
