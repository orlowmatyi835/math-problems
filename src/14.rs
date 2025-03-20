fn main() {
    let mut rng = rand::thread_rng();
    let n1: u32 = rng.gen_range(1..=99);
    let n2: u32 = rng.gen_range(1..=99);
    println!("{} + {}", n1, n2);
}
