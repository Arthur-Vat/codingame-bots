use super::*;

#[test]
fn reads_a_codingame_turn() {
    let mut input = Input::new("-1 -1\n2\n4 4\n0 8\n".as_bytes());
    assert_eq!(input.two::<i32, i32>(), Ok((-1, -1)));
    assert_eq!(input.one::<usize>(), Ok(2));
    assert_eq!(input.two::<i32, i32>(), Ok((4, 4)));
    assert_eq!(input.two::<i32, i32>(), Ok((0, 8)));
    assert_eq!(input.line(), Err(InputError::Eof));
}

#[test]
fn strips_windows_line_endings_and_extra_spaces() {
    let mut input = Input::new("  3 \r\n 7   8 \r\nlast".as_bytes());
    assert_eq!(input.one::<u8>(), Ok(3));
    assert_eq!(input.two::<u8, u8>(), Ok((7, 8)));
    assert_eq!(input.line(), Ok("last"));
}

#[test]
fn reads_several_values() {
    let mut input = Input::new("1 2 3\n\n".as_bytes());
    assert_eq!(input.many::<i64>(), Ok(vec![1, 2, 3]));
    assert_eq!(input.many::<i64>(), Ok(vec![]));
}

#[test]
fn reports_what_was_expected() {
    let mut input = Input::new("4 x\nfive\n1 2 3\n".as_bytes());
    let err = input.two::<i32, i32>().unwrap_err();
    assert!(err.to_string().contains("\"4 x\""), "{err}");
    let err = input.one::<u32>().unwrap_err();
    assert!(err.to_string().contains("one value (u32)"), "{err}");
    assert!(matches!(
        input.two::<i32, i32>(),
        Err(InputError::Parse { .. })
    ));
}
