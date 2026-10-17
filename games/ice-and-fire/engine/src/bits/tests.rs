use cg_core::rng::Rng;

use super::*;

fn random_set(rng: &mut Rng) -> (Bits, [bool; CELLS]) {
    let mut bits = Bits::EMPTY;
    let mut flags = [false; CELLS];
    let density = 1 + rng.below(9);
    for (cell, flag) in flags.iter_mut().enumerate() {
        if rng.below(10) < density {
            *flag = true;
            bits.set(cell);
        }
    }
    (bits, flags)
}

fn from_flags(flags: &[bool; CELLS]) -> Bits {
    let mut bits = Bits::EMPTY;
    for (cell, &flag) in flags.iter().enumerate() {
        if flag {
            bits.set(cell);
        }
    }
    bits
}

#[test]
fn shifts_move_every_cell_one_step_and_drop_those_leaving_the_map() {
    let mut rng = Rng::new(1);
    for _ in 0..500 {
        let (bits, flags) = random_set(&mut rng);
        let mut up = [false; CELLS];
        let mut down = [false; CELLS];
        let mut left = [false; CELLS];
        let mut right = [false; CELLS];
        for cell in (0..CELLS).filter(|&cell| flags[cell]) {
            let (x, y) = (cell % SIZE, cell / SIZE);
            if y > 0 {
                up[cell - SIZE] = true;
            }
            if y + 1 < SIZE {
                down[cell + SIZE] = true;
            }
            if x > 0 {
                left[cell - 1] = true;
            }
            if x + 1 < SIZE {
                right[cell + 1] = true;
            }
        }
        assert_eq!(bits.shift_up(), from_flags(&up));
        assert_eq!(bits.shift_down(), from_flags(&down));
        assert_eq!(bits.shift_left(), from_flags(&left));
        assert_eq!(bits.shift_right(), from_flags(&right));
        let all_sides: [bool; CELLS] =
            std::array::from_fn(|c| up[c] || down[c] || left[c] || right[c]);
        assert_eq!(bits.neighbours(), from_flags(&all_sides));
    }
}

#[test]
fn counting_iterating_and_choosing_agree() {
    let mut rng = Rng::new(2);
    for _ in 0..500 {
        let (bits, flags) = random_set(&mut rng);
        let cells: Vec<usize> = (0..CELLS).filter(|&cell| flags[cell]).collect();
        assert_eq!(bits.count() as usize, cells.len());
        assert_eq!(bits.iter().collect::<Vec<_>>(), cells);
        assert_eq!(bits.is_empty(), cells.is_empty());
        for (n, &cell) in cells.iter().enumerate() {
            assert_eq!(bits.nth(n), Some(cell));
            assert!(bits.test(cell));
        }
        assert_eq!(bits.nth(cells.len()), None);
    }
}

#[test]
fn complement_stays_inside_the_map() {
    assert_eq!((!Bits::EMPTY).count() as usize, CELLS);
    assert_eq!(!Bits::ALL, Bits::EMPTY);
    let one = Bits::cell(143);
    assert_eq!((!one).count() as usize, CELLS - 1);
}

#[test]
fn set_and_clear_touch_one_cell() {
    let mut bits = Bits::EMPTY;
    for cell in [0, 47, 48, 95, 96, 143] {
        bits.set(cell);
        assert!(bits.test(cell));
    }
    assert_eq!(bits.count(), 6);
    bits.clear(48);
    assert!(!bits.test(48));
    assert_eq!(bits.count(), 5);
}
