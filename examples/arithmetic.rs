use aoc::{Day, Example, Part, Runner, Solver};

struct SolverOne;

impl Solver<u32> for SolverOne {
    fn solve(&self, input: &str) -> Result<u32, String> {
        let mut sum = 0;

        for raw_number in input.split(' ') {
            let number: u32 = raw_number
                .parse()
                .map_err(|_| format!("failed to parse '{}' as a number.", raw_number))?;
            sum += number;
        }

        Ok(sum)
    }
}

struct SolverTwo;

impl Solver<u32> for SolverTwo {
    fn solve(&self, input: &str) -> Result<u32, String> {
        let mut product = 1;

        for raw_number in input.split(' ') {
            let number: u32 = raw_number
                .parse()
                .map_err(|_| format!("failed to parse '{}' as a number.", raw_number))?;
            product *= number;
        }

        Ok(product)
    }
}

fn main() {
    let example_one = Example::new("12 3", 15);
    let example_two = Example::new("12 3", 36);

    let part_one = Part::new(&SolverOne, example_one);
    let part_two = Part::new(&SolverTwo, example_two);

    let day = Day::new(part_one, part_two, "7 13 5");

    let runner = Runner::new(day);
    runner.run_part_one_example();
    eprintln!();
    runner.run_part_two_example();
    eprintln!();
    runner.run_part_one();
    eprintln!();
    runner.run_part_two();
}
