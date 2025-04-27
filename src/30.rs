// Solution 1: Implement a simple fibonacci sequence generator
fn main() {
    let n = 10; // Change this value to generate different sequences
    let (a, b) = calculate_fibonacci(n);
    println!("Fibonacci sequence up to {}:", n);
    for i in 0..=n {
        match a <= b {
            true => println!("{}", a),
            false => println!("{}", b),
        }
        a = b;
        b = a + b;
    }
}

// Solution 2: Implement a simple prime number generator
fn main() {
    let n = 10; // Change this value to generate different sequences
    let (prime, _) = find_prime(n);
    println!("Prime numbers up to {}:", n);
    for i in 0..=n {
        match prime <= i {
            true => println!("{}", i),
            false => break;
        }
    }
}

// Solution 3: Implement a simple arithmetic sequence generator
fn main() {
    let (a, b) = calculate_arithmetic_sequence(5, 2);
    println!("Arithmetic sequence up to {}:", 5);
    for i in 0..=5 {
        match a <= b {
            true => println!("{}", a),
            false => break,
        }
        a = b;
        b = a + b;
    }
}

// Solution 4: Implement a simple geometric sequence generator
fn main() {
    let (a, b) = calculate_geometric_sequence(2, 5);
    println!("Geometric sequence up to {}:", 2);
    for i in 0..=2 {
        match a <= b {
            true => println!("{}", a),
            false => break,
        }
        a = b;
        b = a * b;
    }
}

// Solution 5: Implement a simple prime and composite number generator
fn main() {
    let (prime, composites) = find_prime_and_composites(10);
    println!("Composite numbers up to {}:", 10);
    for i in 0..=10 {
        match prime <= i && !composites.contains(&i) {
            true => println!("{}", i),
            false => break,
        }
    }
}

// Solution 6: Implement a simple arithmetic and geometric sequence generator
fn main() {
    let (a, b) = calculate_arithmetic_geometric_sequence(10, 2);
    println!("Arithmetic and geometric sequence up to {}:", 10);
    for i in 0..=10 {
        match a <= b {
            true => println!("{}", a),
            false => break,
        }
        a = b;
        b = a * b;
    }
}
