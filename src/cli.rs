use std::fmt::Display;

use clap::{Arg, Command};
use colored::Colorize;

use crate::Day;

fn get_cli(command_name: String) -> Command {
    Command::new(command_name)
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .subcommand_required(true)
        .subcommand(
            Command::new("solve").arg(
                Arg::new("part")
                    .help("the part(s) to solve.")
                    .long("part")
                    .value_parser(["1", "2", "both"])
                    .default_value("both"),
            ),
        )
        .subcommand(
            Command::new("check").arg(
                Arg::new("part")
                    .help("the part(s) to check.")
                    .long("part")
                    .value_parser(["1", "2", "one", "two", "both"])
                    .default_value("both"),
            ),
        )
}

pub fn run<O1, O2>(day: Day<O1, O2>)
where
    O1: Display + PartialEq,
    O2: Display + PartialEq,
{
    let command = get_cli(day.get_day_description());
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("solve", matches)) => match matches.get_one::<String>("part").unwrap().as_str() {
            "both" => {
                eprintln!("solving '{}'", day.get_solver_one_name());
                print_solve_output(day.solve_one());
                eprintln!();
                eprintln!("solving '{}'", day.get_solver_two_name());
                print_solve_output(day.solve_two());
            }
            "1" | "one" => {
                eprintln!("solving '{}'", day.get_solver_one_name());
                print_solve_output(day.solve_one());
            }
            "2" | "two" => {
                eprintln!("solving '{}'", day.get_solver_two_name());
                print_solve_output(day.solve_two());
            }
            _ => unreachable!(),
        },
        Some(("check", matches)) => match matches.get_one::<String>("part").unwrap().as_str() {
            "both" => {
                eprintln!("checking '{}'", day.get_solver_one_name());
                print_check_output(day.check_one());
                eprintln!();
                eprintln!("checking '{}'", day.get_solver_two_name());
                print_check_output(day.check_two());
            }
            "1" | "one" => {
                eprintln!("solving '{}'", day.get_solver_one_name());
                print_check_output(day.check_one());
            }
            "2" | "two" => {
                eprintln!("solving '{}'", day.get_solver_two_name());
                print_check_output(day.check_two());
            }
            _ => unreachable!(),
        },
        _ => unreachable!("the cli requires a subcommand"),
    }
}

fn print_solve_output<O>(output: Result<O, String>)
where
    O: Display,
{
    match output {
        Ok(output) => {
            eprintln!("{}", "success".green());
            eprintln!("===== start of output =====");
            println!("{}", output);
            eprintln!("===== end of output =====");
        }
        Err(e) => eprintln!("{}", format!("an error occurred: {}", e).red()),
    }
}

fn print_check_output<O>(output: Result<(bool, O), String>)
where
    O: Display,
{
    match output {
        Ok((true, _)) => {
            eprintln!("{}", "success".green());
            eprintln!(
                "{}",
                "your solver calculated the correct output for the example"
            );
        }
        Ok((false, output)) => {
            eprintln!("{}", "error".red());
            eprintln!(
                "{}",
                "your solver did not calculate the correct output for the example".red()
            );
            eprintln!("===== start of output =====");
            eprintln!("{}", output);
            eprintln!("===== end of output =====");
        }
        Err(e) => eprintln!("{}", format!("an error occurred: {}", e).red()),
    }
}
