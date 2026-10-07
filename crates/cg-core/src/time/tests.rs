use super::*;

#[test]
fn parses_the_time_scale() {
    assert_eq!(parse_time_scale(Some("0.2")), 0.2);
    assert_eq!(parse_time_scale(Some(" 3 ")), 3.0);
    assert_eq!(parse_time_scale(None), 1.0);
    for invalid in ["", "fast", "0", "-1", "NaN", "inf"] {
        assert_eq!(parse_time_scale(Some(invalid)), 1.0, "{invalid:?}");
    }
}
