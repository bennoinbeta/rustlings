// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

#[test]
fn indexing_tuple() {
    let numbers: (u8, u8, u8) = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second: &u8 = &numbers.1;

    // With {:p} we tell the println macro to print the pointer address
    // and not dereference the reference under the hood
    println!("Pointer: {:p}", second);

    assert_eq!(2, *second, "This is not the 2nd number in the tuple!")
}
