// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0: Vec<i32> = vec![22, 44, 66];

    // Instead of moving the ownerhip of vec0 into 'fill_vec()' we just pass a reference to it
    // and then clone it in 'fill_vec()' in order to have a seperate instance.
    // Since we only passed a reference of vec0 into 'fill_vec()' we cann still use it after the function call.
    let mut vec1 = fill_vec(&vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec.clone();

    vec.push(88);

    vec
}
