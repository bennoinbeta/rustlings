// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number: &str = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // Redeclare variable number and thus shadow it (Shadowing).
    // The number of type u8 shadows the number of type &str,
    // so that anything past the declaration of number of type u8
    // will refer to the number of type u8 (so the number that shadowed the other number).
    let number: u8 = 3; 
    println!("Number plus two is : {}", number + 2);
}
