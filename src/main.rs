mod problems;

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let problem_map: HashMap<usize, fn() -> String> = [
        (1, problems::problem001::subject as fn() -> String),
        (2, problems::problem002::subject as fn() -> String),
    ]
    .into_iter()
    .collect();

    match args.len() {
        1 => run_all_problems(&problem_map),
        2 => match args[1].parse::<usize>() {
            Ok(problem_number) => run_specific_problem(problem_number, &problem_map),
            Err(_) => argument_error("Error: Argument is not a valid number."),
        },
        _ => argument_error("Error: Invalid number of arguments."),
    }
}

fn argument_error(message: &str) {
    eprintln!("{}", message);
    eprintln!("Usage:");
    eprintln!("  cargo run              # Runs all problems");
    eprintln!("  cargo run -- <number>  # Runs the specific problem with the given number");
    std::process::exit(1);
}

fn run_and_print(problem_number: usize, solve_function: &fn() -> String) {
    println!("Problem {:03} | {}", problem_number, solve_function());
}

fn run_all_problems(problem_map: &HashMap<usize, fn() -> String>) {
    let mut sorted_keys: Vec<&usize> = problem_map.keys().collect();
    sorted_keys.sort();
    for number in sorted_keys {
        if let Some(solve_function) = problem_map.get(number) {
            run_and_print(*number, solve_function);
        }
    }
}

fn run_specific_problem(number: usize, problem_map: &HashMap<usize, fn() -> String>) {
    match problem_map.get(&number) {
        Some(solve_function) => run_and_print(number, &solve_function),
        None => argument_error(&format!("Problem {} is not implemented yet.", number)),
    }
}
