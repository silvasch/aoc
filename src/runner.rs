use std::fmt::Display;

use colored::Colorize;

use crate::Day;

pub struct Runner<'a, O1, O2>
where
    O1: PartialEq + Display,
    O2: PartialEq + Display,
{
    day: Day<'a, O1, O2>,
}

impl<'a, O1, O2> Runner<'a, O1, O2>
where
    O1: PartialEq + Display,
    O2: PartialEq + Display,
{
    pub fn new(day: Day<'a, O1, O2>) -> Self {
        Self { day }
    }

    pub fn run_part_one(&self) {
        eprintln!("Getting the solution for part one...");
        match self.day.get_part_one_solution() {
            Ok(solution) => {
                eprintln!("{}", "...done!".green());
                eprintln!("===== Start of Output =====");
                println!("{}", solution);
                eprintln!("===== End of Output =====");
            }
            Err(e) => {
                eprintln!("{}{}", "An error occurred: ".red(), e.red());
            }
        }
    }

    pub fn run_part_two(&self) {
        eprintln!("Getting the solution for part two...");
        match self.day.get_part_two_solution() {
            Ok(solution) => {
                eprintln!("{}", "...done!".green());
                eprintln!("===== Start of Output =====");
                println!("{}", solution);
                eprintln!("===== End of Output =====");
            }
            Err(e) => {
                eprintln!("{}{}", "An error occurred: ".red(), e.red());
            }
        }
    }

    pub fn run_part_one_example(&self) {
        eprintln!("Checking your solution for part one...");
        match self.day.check_part_one_example() {
            Ok((result, output)) => {
                if result {
                    eprintln!("{}", "Your solution solved the example correctly!".green());
                } else {
                    eprintln!(
                        "{}",
                        "Your solution did not solve the example correctly!".red()
                    );
                    eprintln!("===== Expected Output =====");
                    eprintln!("{}", self.day.part_one.example.expected_output);
                    eprintln!("===== End of Expected Output =====");
                    eprintln!("===== Your Output =====");
                    eprintln!("{}", output);
                    eprintln!("===== End of Your Output =====");
                }
            }
            Err(e) => {
                eprintln!("{}{}", "An error occurred: ".red(), e.red());
            }
        }
    }

    pub fn run_part_two_example(&self) {
        eprintln!("Checking your solution for part two...");
        match self.day.check_part_two_example() {
            Ok((result, output)) => {
                if result {
                    eprintln!("{}", "Your solution solved the example correctly!".green());
                } else {
                    eprintln!(
                        "{}",
                        "Your solution did not solve the example correctly!".red()
                    );
                    eprintln!("===== Expected Output =====");
                    eprintln!("{}", self.day.part_one.example.expected_output);
                    eprintln!("===== End of Expected Output =====");
                    eprintln!("===== Your Output =====");
                    eprintln!("{}", output);
                    eprintln!("===== End of Your Output =====");
                }
            }
            Err(e) => {
                eprintln!("{}{}", "An error occurred: ".red(), e.red());
            }
        }
    }
}
