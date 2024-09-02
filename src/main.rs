mod math;
mod problems;

use crate::problems::PROBLEMS;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => run_all_problems(),
        2 => match args[1].parse::<usize>() {
            Ok(problem_number) => run_specific_problem(problem_number),
            Err(_) => argument_error("Error: Argument is not a valid number."),
        },
        _ => argument_error("Error: Invalid number of arguments."),
    }
}

fn argument_error(message: &str) {
    eprintln!("{}", message);
    eprintln!("Usage:");
    eprintln!("  cargo run              # Runs all problems");
    eprintln!("  cargo run -- <number>  # Runs the problem with the given number");
    std::process::exit(1);
}

fn run_and_print(problem_number: usize, solve_function: &fn() -> String) {
    println!("Problem {:03} | {}", problem_number, solve_function());
}

fn run_all_problems() {
    let mut sorted_keys: Vec<&usize> = PROBLEMS.keys().collect();
    sorted_keys.sort();
    for number in sorted_keys {
        if let Some(solve_function) = PROBLEMS.get(number) {
            run_and_print(*number, solve_function);
        }
    }
}

fn run_specific_problem(number: usize) {
    match PROBLEMS.get(&number) {
        Some(solve_function) => run_and_print(number, &solve_function),
        None => argument_error(&format!("Problem {} is not implemented yet.", number)),
    }
}
