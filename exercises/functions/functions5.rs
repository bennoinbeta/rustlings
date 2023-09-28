// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer: i32 = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    // num * num; // Is interpreted as expression in a function statement but not return value thus this method would return a Unit () which is like null / undefined in Rust
    num * num
    // or
    // return num * num;
}
