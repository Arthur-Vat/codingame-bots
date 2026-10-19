use super::*;

/// Per seed (unsigned 64-bit), six groups of `below(12)`, `below(16)`,
/// `below(7)` and the bits of `fraction()`, printed by Java.
type Group = (i32, i32, i32, u32);

const VECTORS: [(u64, [Group; 6]); 6] = [
    (
        0,
        [
            (0, 13, 4, 1058748784),
            (11, 4, 2, 1039114536),
            (3, 12, 1, 1048669152),
            (5, 9, 4, 1065064975),
            (4, 0, 6, 1043608384),
            (7, 5, 3, 1052534608),
        ],
    ),
    (
        1,
        [
            (9, 1, 1, 1053858802),
            (2, 0, 0, 1059629957),
            (10, 11, 5, 1042048736),
            (1, 2, 2, 1057871790),
            (8, 14, 0, 1056531332),
            (8, 14, 4, 1042491880),
        ],
    ),
    (
        42,
        [
            (2, 0, 6, 1027890176),
            (6, 15, 1, 1060449413),
            (7, 1, 3, 1055329070),
            (0, 6, 1, 1060159422),
            (0, 12, 0, 1065322657),
            (8, 2, 1, 1054944624),
        ],
    ),
    (
        18446744073709551615,
        [
            (5, 7, 4, 1057770495),
            (8, 9, 6, 1053585290),
            (1, 0, 5, 1050581462),
            (9, 1, 6, 1060153908),
            (9, 14, 1, 1060849005),
            (5, 5, 5, 1055429502),
        ],
    ),
    (
        123456789,
        [
            (1, 12, 6, 1048739396),
            (6, 3, 0, 1054464466),
            (1, 6, 5, 1056640406),
            (0, 0, 5, 1056235380),
            (4, 1, 0, 1059201119),
            (6, 8, 1, 1053895528),
        ],
    ),
    (
        9223372036854775808,
        [
            (0, 13, 4, 1058748784),
            (11, 4, 2, 1039114536),
            (3, 12, 1, 1048669152),
            (5, 9, 4, 1065064975),
            (4, 0, 6, 1043608384),
            (7, 5, 3, 1052534608),
        ],
    ),
];

#[test]
fn it_draws_the_numbers_that_java_printed() {
    for (seed, groups) in VECTORS {
        let mut random = JavaRandom::new(seed as i64);
        for (group, &expected) in groups.iter().enumerate() {
            let drawn = (
                random.below(12),
                random.below(16),
                random.below(7),
                random.fraction().to_bits(),
            );
            assert_eq!(drawn, expected, "seed {seed}, group {group}");
        }
    }
}

#[test]
fn fractions_are_in_zero_to_one() {
    let mut random = JavaRandom::new(-5);
    for _ in 0..10_000 {
        let fraction = random.fraction();
        assert!((0.0..1.0).contains(&fraction));
    }
}

#[test]
fn a_draw_below_one_gives_zero_and_takes_its_bits() {
    let mut random = JavaRandom::new(42);
    let mut fresh = JavaRandom::new(42);
    assert_eq!(random.below(1), 0);
    fresh.below(12);
    assert_eq!(random.below(1_000_003), fresh.below(1_000_003));
}
