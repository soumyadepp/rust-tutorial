pub fn declare_variables() {
    let name = "Soumyadeep";
    let mut age = 25;
    age += 1; // Increment age
    // name = "Soumyadeep Saha"; // This line will cause an error because `name` is immutable
    println!("Name: {}, Age: {}", name, age);
}