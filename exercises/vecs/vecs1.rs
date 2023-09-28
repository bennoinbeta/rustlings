// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

fn array_and_vec() -> ([u8; 4], Vec<u8>) {
    let a: [u8; 4] = [10, 20, 30, 40]; // a plain array
    let v: Vec<u8> = vec![10, 20, 30, 40];
    let v2: Vec<u8> = Vec::from([10, 20, 30, 40]);
    let mut v3: Vec<u8> = Vec::new();
    v3.extend(&v);
    let v4: Vec<u8> = a.into();

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
