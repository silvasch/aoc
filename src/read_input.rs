use std::str::FromStr;

pub fn read_input(day: u32) -> Result<String, ReadInputError<String>> {
    std::fs::read_to_string(format!(
        "{}/inputs/day{:0>2}/input.txt",
        location_macros::crate_dir!(),
        day
    ))
    .map(|v| v.trim().to_string())
    .map_err(|e| ReadInputError::ReadInput(e))
}

pub fn read_example_input(day: u32, part: u32) -> Result<String, ReadInputError<String>> {
    std::fs::read_to_string(format!(
        "{}/inputs/day{:0>2}/part{:0>2}-example-input.txt",
        location_macros::crate_dir!(),
        day,
        part
    ))
    .map(|v| v.trim().to_string())
    .map_err(|e| ReadInputError::ReadInput(e))
}

pub fn read_example_output<O: std::fmt::Debug + FromStr>(
    day: u32,
    part: u32,
) -> Result<O, ReadInputError<O>> {
    std::fs::read_to_string(format!(
        "{}/inputs/day{:0>2}/part{:0>2}-example-output.txt",
        location_macros::crate_dir!(),
        day,
        part
    ))
    .map_err(|e| ReadInputError::ReadInput(e))?
    .trim()
    .parse()
    .map_err(|e| ReadInputError::ParseInput(e))
}

#[derive(Debug)]
pub enum ReadInputError<O>
where
    O: std::fmt::Debug + FromStr,
{
    ReadInput(std::io::Error),
    ParseInput(O::Err),
}

trait DebugDisplay: std::fmt::Debug + std::fmt::Display {}

impl<O> std::fmt::Display for ReadInputError<O>
where
    O: std::fmt::Debug + FromStr<Err = dyn DebugDisplay>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadInputError::ReadInput(e) => write!(f, "{}", e),
            ReadInputError::ParseInput(e) => write!(f, "{}", e),
        }
    }
}

impl<O> std::error::Error for ReadInputError<O> where
    O: std::fmt::Debug + FromStr<Err = dyn DebugDisplay>
{
}
