use aoc::{run, Day, Example, Part, Solver};

const INPUT: &str = "12 -3 23";

struct SolverOne;

impl<'a> Solver<'a, i32> for SolverOne {
    fn name(&self) -> &'a str {
        "Addition"
    }

    fn solve(&self, input: &str) -> Option<Result<i32, String>> {
        let mut sum = 0;
        for raw_number in input.split(' ') {
            let number = match raw_number
                .parse::<i32>()
                .map_err(|_| format!("failed to parse '{}' as a number", raw_number))
            {
                Ok(number) => number,
                Err(e) => return Some(Err(e)),
            };
            sum += number;
        }
        return Some(Ok(sum));
    }
}

struct SolverTwo;

impl<'a> Solver<'a, i32> for SolverTwo {
    fn name(&self) -> &'a str {
        "Multiplication"
    }

    fn solve(&self, input: &str) -> Option<Result<i32, String>> {
        let mut product = 1;
        for raw_number in input.split(' ') {
            let number = match raw_number
                .parse::<i32>()
                .map_err(|_| format!("failed to parse '{}' as a number", raw_number))
            {
                Ok(number) => number,
                Err(e) => return Some(Err(e)),
            };
            product *= number;
        }
        return Some(Ok(product));
    }
}

fn main() {
    let example_one = Example::new("12 4", 16);
    let example_two = Example::new("12 2", 24);

    let part_one = Part::new(&SolverOne, example_one);
    let part_two = Part::new(&SolverTwo, example_two);

    let day = Day::new(1, "Arithmetic", INPUT, part_one, part_two);

    run(day);
}
