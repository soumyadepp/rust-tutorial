fn greet(name: &str) -> String {
    format!("Hello, {}", name)
}

pub fn call_greet() {
    let msg = greet("Soumyadeep");
    println!("{}", msg);
}
