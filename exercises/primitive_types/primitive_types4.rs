// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a: [u8; 5] = [1, 2, 3, 4, 5];

    // **Reference** to contiguous sequence of elements in a **collection**
    // Provide a way to **borrow part** of a collection without taking **ownership** of the entire collection (array, vec, String)
    let nice_slice: &[u8] = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
