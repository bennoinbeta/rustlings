// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // if value.is_negative() {
        //     Err(CreationError::Negative)
        // } else if value == 0 {
        //     Err(CreationError::Zero)
        // } else {
        //     Ok(PositiveNonzeroInteger(value as u64))
        // }

        // or

        match value {
            num if num.is_negative() => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            num => Ok(PositiveNonzeroInteger(num as u64)),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
