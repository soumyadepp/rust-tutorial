pub fn error() {
    let s1 = String::from("Hello, Rust!");
    let s2 = s1;
    // println!("{}", s1); // This line would cause an error because s1 is no longer valid after the move
    println!("{}", s2); // This will print "Hello, Rust!"
}

pub fn correct() {
    let s1 = String::from("Hello, Rust!");
    let s2 = &s1; // Borrowing s1
    println!("{}", s2); // This will print "Hello, Rust!"
    
    // Ownership transfer example
    let s3 = s1; // s1 is moved to s3
    // println!("{}", s1); // This line would cause an error because s1 is no longer valid after the move
    println!("{}", s3); // This will print "Hello, Rust!"
}