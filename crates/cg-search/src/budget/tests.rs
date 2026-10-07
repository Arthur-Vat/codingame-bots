use super::*;

#[test]
fn search_time_scales_the_limit_and_keeps_a_reserve() {
    let limit = Duration::from_millis(100);
    let reserve = Duration::from_millis(2);
    assert_eq!(
        search_time(limit, 1.0, 0.9, reserve),
        Duration::from_millis(88)
    );
    assert_eq!(
        search_time(limit, 0.2, 0.9, reserve),
        Duration::from_millis(16)
    );
    // A reserve larger than the share leaves no time rather than panicking.
    assert_eq!(search_time(limit, 0.01, 0.9, reserve), Duration::ZERO);
}

#[test]
fn fixed_iterations_must_be_a_positive_integer() {
    assert_eq!(fixed_iterations(Some("5000")), Some(5000));
    assert_eq!(fixed_iterations(Some(" 7 ")), Some(7));
    assert_eq!(fixed_iterations(Some("0")), None);
    assert_eq!(fixed_iterations(Some("many")), None);
    assert_eq!(fixed_iterations(None), None);
}

#[test]
fn budgets_stop_on_time_or_count() {
    assert!(Budget::Iterations(10).is_spent(10));
    assert!(!Budget::Iterations(10).is_spent(9));
    assert!(Budget::Until(Instant::now()).is_spent(0));
    let later = Instant::now() + Duration::from_secs(60);
    assert!(!Budget::Until(later).is_spent(u64::MAX));
}
