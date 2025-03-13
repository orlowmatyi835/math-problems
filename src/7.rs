
let mut rng = rand::thread_rng();

fn solve_math_problem() -> i32 {
    let problem = rng.gen_range(1, 5); // Select a random math problem from the set of problems

    match problem {
        1 => 5 + 2,                   // Addition
        2 => 4 * 6,                   // Multiplication
        3 => 7 - 3,                   // Subtraction
        4 => 9 / 3,                   // Division
        _ => 0,                       // Default case (should never happen)
    }
}