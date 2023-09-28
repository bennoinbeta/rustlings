// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // Array is a fixed sized element and can't grow or shrink at runtime
    // Here we create an array with 100 elements filled with 1 and below with 'b'
    let a: [u8; 100] = [1; 100];
    let b: [char; 100] = ['b'; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
