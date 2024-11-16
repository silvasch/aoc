use aoc::{Day, Example, Part, Runner, Solver};

const INPUT: &str = "13 4 12";

struct SolverOne;

impl<'a> Solver<'a, i32> for SolverOne {
    fn solve(&self, input: &str) -> Option<Result<i32, String>> {
        let mut sum = 0;
        for raw_number in input.split(' ') {
            match raw_number.parse::<i32>() {
                Ok(number) => sum += number,
                Err(_) => {
                    return Some(Err(format!("failed to parse '{}' as a number", raw_number)))
                }
            }
        }
        Some(Ok(sum))
    }

    fn name(&self) -> &'a str {
        "addition"
    }
}

struct SolverTwo;

impl<'a> Solver<'a, i32> for SolverTwo {
    fn solve(&self, input: &str) -> Option<Result<i32, String>> {
        let mut product = 1;
        for raw_number in input.split(' ') {
            match raw_number.parse::<i32>() {
                Ok(number) => product *= number,
                Err(_) => {
                    return Some(Err(format!("failed to parse '{}' as a number", raw_number)))
                }
            }
        }
        Some(Ok(product))
    }

    fn name(&self) -> &'a str {
        "multiplication"
    }
}

fn main() {
    let example_one = Example::new("3 4", 7);
    let example_two = Example::new("2 5", 10);

    let part_one = Part::new(&SolverOne, example_one);
    let part_two = Part::new(&SolverTwo, example_two);

    let day = Day::new(INPUT, part_one, part_two);

    let runner = Runner::new(day);
    runner.run();
}
