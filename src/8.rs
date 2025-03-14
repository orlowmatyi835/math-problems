use std::{collections::HashMap, hash::Hash};

fn generate_random_number(min: i32, max: i32) -> i32 {
    min + (max - min) * rand::random()
}

fn solve_equation(coefficients: HashMap<i32, i32>) {
    let mut x = generate_random_number(0, 10);
    let y = coefficients.iter().fold(0, |acc, (power, coeff)| acc + coeff * x.pow(power));
    println!("x: {}", x);
    println!("y: {}", y);
}
