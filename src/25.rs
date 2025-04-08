fn main() {
    // Example code to demonstrate Rust's unique features and idioms

    // Simple variable assignment
    let x = 5;
    println!("Value of x: {}", x);

    // Arithmetic operations
    let y = 10 + 2;
    println!("Result of addition: {}", y);
    
    // Bitwise operations
    let a = 128; // 32 bit integer
    let b = 4096; // 32 bit integer
    let c = a & b; // Perform bitwise AND operation
    println!("Bitwise and result of a and b: {}", c);

    // Comparison operators
    let d = 10 > 5;
    println!("Value of d: {}", d);

    // Logical operators
    let e = !false;
    println!("Value of e (not false): {}", e);
}
