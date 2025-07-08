mod variables;
mod ownership_and_borrowing;
mod functions_and_return_types;
mod structs_and_impls;

use crate::variables::declare_variables;
use crate::ownership_and_borrowing::error;
use crate::ownership_and_borrowing::correct;
use crate::functions_and_return_types::call_greet;
use crate::structs_and_impls::Message;

// Main function to run the program
// This function is the entry point of the Rust program
fn main() {
    println!("Hello, Rust!");

    // Call the function to declare and print variables
    // This will execute the code in the `declare_variables` function
    // which is defined in the `variables` module
    // This function demonstrates variable declaration and mutability in Rust
    // It will print the name and age to the console
    // The `name` variable is immutable, while `age` is mutable
    println!("----------------------------------------");
    declare_variables();
    println!("----------------------------------------");

    // Call the function to demonstrate ownership and borrowing
    // This will execute the code in the `ownership_and_borrowing` function
    // which is defined in the `ownership_and_borrowing` module
    // This function demonstrates how ownership works in Rust
    // It will print a string to the console
    error();
    println!("----------------------------------------");
    // Call the function to demonstrate correct ownership and borrowing
    // This will execute the code in the `correct` function
    // which is defined in the `ownership_and_borrowing` module
    // This function demonstrates how to correctly borrow and transfer ownership in Rust
    correct();
    println!("----------------------------------------");
    // Call the function to greet a user using a function with specified return type.
    // This will execute the code in the `call_greet` function
    // which is defined in the `functions_and_return_types` module
    // This function demonstrates how to define and call functions in Rust
    call_greet();
    println!("----------------------------------------"); 
    // Create a new instance of the Message struct
    // This will execute the code in the `Message` struct's `new` method
    // which is defined in the `structs_and_impls` module
    let message = Message::new("greeting", "Hello, Rust!");
    // Call the `print` method on the `message` instance
    // This will execute the code in the `print` method of the `Message` struct
    // which is defined in the `structs_and_impls` module
    message.print();
    println!("----------------------------------------");
}
