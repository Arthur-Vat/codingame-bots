use super::*;

#[test]
fn matches_the_xoshiro256plusplus_reference() {
    // Reference outputs for the state [1, 2, 3, 4].
    let mut rng = Rng {
        state: [1, 2, 3, 4],
    };
    let expected = [41943041, 58720359, 3588806011781223, 3591011842654386];
    for value in expected {
        assert_eq!(rng.next_u64(), value);
    }
}

#[test]
fn same_seed_same_sequence() {
    // Computed independently with a Python implementation of SplitMix64
    // seeding followed by xoshiro256++.
    let mut rng = Rng::new(42);
    assert_eq!(rng.next_u64(), 15021278609987233951);
    assert_eq!(rng.next_u64(), 5881210131331364753);
    assert_eq!(rng.next_u64(), 18149643915985481100);
    let mut a = Rng::new(7);
    let mut b = Rng::new(7);
    assert!((0..100).all(|_| a.next_u64() == b.next_u64()));
}

#[test]
fn below_stays_in_range_and_covers_it() {
    let mut rng = Rng::new(1);
    let mut seen = [0u32; 7];
    for _ in 0..7000 {
        let value = rng.below(7);
        assert!(value < 7);
        seen[value as usize] += 1;
    }
    // Each value should appear about 1000 times.
    assert!(
        seen.iter().all(|&count| (850..1150).contains(&count)),
        "{seen:?}"
    );
    assert_eq!(rng.below(1), 0);
}

#[test]
#[should_panic(expected = "no possible value")]
fn below_zero_panics() {
    Rng::new(1).below(0);
}

#[test]
fn unit_is_in_zero_one() {
    let mut rng = Rng::new(3);
    assert!((0..1000)
        .map(|_| rng.unit())
        .all(|x| (0.0..1.0).contains(&x)));
}

#[test]
fn pick_and_shuffle() {
    let mut rng = Rng::new(5);
    assert_eq!(rng.pick::<u8>(&[]), None);
    assert_eq!(rng.pick(&[9]), Some(&9));

    let mut items: Vec<u32> = (0..50).collect();
    rng.shuffle(&mut items);
    assert_ne!(items, (0..50).collect::<Vec<_>>());
    items.sort_unstable();
    assert_eq!(items, (0..50).collect::<Vec<_>>());
}

#[test]
fn parses_the_seed_variable() {
    // The parsing is tested on its own: changing the process environment
    // would race with other tests.
    assert_eq!(parse_seed(Some(" 12 ")), Some(12));
    assert_eq!(parse_seed(Some("abc")), None);
    assert_eq!(parse_seed(None), None);
}
