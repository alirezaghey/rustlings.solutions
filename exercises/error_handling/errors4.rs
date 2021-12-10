// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    // Using match construct
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            // Below line is a way to match infinite/open ranges
            // i64::MIN..=-1_i64 => Err(CreationError::Negative),
            v if { v < 0 } => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            _ => Ok(PositiveNonzeroInteger(value as u64)),
        }
    }
    // Using standard if else statements
    // fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
    //     if value < 0 {
    //         Err(CreationError::Negative)
    //     } else if value == 0 {
    //         Err(CreationError::Zero)
    //     } else {
    //     Ok(PositiveNonzeroInteger(value as u64))
    //     }
    // }
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
