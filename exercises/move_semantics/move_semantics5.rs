// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// fn main() {
//     let mut x: i32 = 100;

//     // This breaks one of the rule of borrowing:
//     // At any given time, you can have either one mutable reference or any number of immutable references!
//     //
//     // Because we try to have two mutable references in the same scope (main function statement / block of code) here,
//     // Rust won't compile.
//     // If y has exclusive mutable access then z can't also have exclusive access to x at the same time!
//     //
//     // To fix that we could create multipe statements in the main function statement
//     // and thus reduce the lifetime of the mutable reference to the statement scope it lives in.
//     // Since when the owner goes out of scope, the value will be dropped.
//     let y: &mut i32 = &mut x;
//     let z: &mut i32 = &mut x;

//     *y += 100;
//     *z += 1000;
//     assert_eq!(x, 1200);
// }

#[test]
fn main() {
    let mut x: i32 = 100;
    {
        let y: &mut i32 = &mut x;
        *y += 100;
    } // y goes out of scope here, so we can make a new reference with no problems
    {
        let z: &mut i32 = &mut x;
        *z += 1000;
    }

    assert_eq!(x, 1200);
}
