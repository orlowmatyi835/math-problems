use rand::{self, Rng};

fn main() {
    let num1: i32 = rand::thread_rng().gen_range(0..100);
    let num2: i32 = rand::thread_rng().gen_range(0..100);

    println!("What is {} + {}?", num1, num2);
}
