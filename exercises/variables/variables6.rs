// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

// Constants are always immutable and they are declared with the 'const' keyword.
// Constants type must always be annotated.
// Constants are valid for the entire time a program runs, within the scope in which they were declared.
const NUMBER: u8 = 3;
fn main() {
    // Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
    // Thats because const is computed at compile time rather than being executed at runtime 
    // so e.g. NUMBER will always be 3
    let mut number: u8 = 10;
    number += 5;
    // const NUMBER_2: u8 = number; // ERROR!

    println!("Number {} - {}", NUMBER, number);
}
