fn main() {
    let mut rng = rand::thread_rng();
    let number1 = rng.gen_range(0..=10);
    let number2 = rng.gen_range(0..=10);
    println!("{} + {} =", number1, number2);
}
