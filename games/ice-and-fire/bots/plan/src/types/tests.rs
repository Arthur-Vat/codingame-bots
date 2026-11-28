use ice_and_fire_engine::State;

use super::*;
use crate::tests::position;

/// A position as red (to act) sees it, at game turn 10: `cells` set on an
/// empty map, which has the two HQs. `(x, y, symbol)` with the symbols of
/// `tests::position`.
fn board(cells: &[(usize, usize, u8)], gold: [i64; 2], units: &[(u8, u32, u8, u8, u8)]) -> State {
    let mut rows = vec![vec![b'.'; 12]; 12];
    rows[0][0] = b'O';
    rows[11][11] = b'X';
    for &(x, y, symbol) in cells {
        rows[y][x] = symbol;
    }
    let rows: Vec<String> = rows
        .into_iter()
        .map(|r| String::from_utf8(r).expect("ascii"))
        .collect();
    let refs: [&str; 12] = std::array::from_fn(|i| rows[i].as_str());
    position(refs, gold, units)
}

/// Red holds the column x = 0 down to (0, 10), a block of 3 by 3 at the
/// bottom of it (x 0 to 2, y 7 to 9) and (1, 1); blue holds the whole row
/// 11. Red's joints are (0, 1) to (0, 7), which join the block to the HQ,
/// and (0, 9), which joins the leaf (0, 10) to the block.
fn line(gold: [i64; 2], units: &[(u8, u32, u8, u8, u8)]) -> State {
    let mut cells = vec![(1, 1, b'O')];
    for y in 1..=10 {
        cells.push((0, y, b'O'));
    }
    for x in 0..=2 {
        for y in 7..=9 {
            cells.push((x, y, b'O'));
        }
    }
    for x in 0..12 {
        cells.push((x, 11, b'X'));
    }
    board(&cells, gold, units)
}

fn cell(x: usize, y: usize) -> usize {
    y * 12 + x
}

fn key_at(state: &State, x: usize, y: usize) -> Key {
    Typing::new(state)
        .key(state, cell(x, y))
        .expect("a unit of the side to act there")
}

#[test]
fn a_unit_next_to_the_enemys_land_is_a_pusher() {
    let state = line([0, 0], &[(0, 1, 1, 0, 10), (1, 2, 1, 1, 11)]);
    let key = key_at(&state, 0, 10);
    assert_eq!(key.typ, Type::Pusher);
    assert_eq!((key.d, key.level), (1, 1));
    // The level-1 unit may enter the empty enemy cell next to it, and an
    // enemy unit is 2 steps away.
    assert_eq!((key.cap, key.en2, key.own2, key.tw2), (1, 1, 0, 0));
}

#[test]
fn a_unit_on_a_joint_near_the_enemy_guards() {
    let state = line([0, 0], &[(0, 1, 1, 0, 9)]);
    let typing = Typing::new(&state);
    let joints = joints(&state, 0);
    assert!(joints.test(cell(0, 9)) && joints.test(cell(0, 1)) && joints.test(cell(0, 7)));
    // A leaf of the column, and the HQ, are not joints.
    assert!(!joints.test(cell(0, 10)) && !joints.test(cell(0, 0)));
    // Cells of the 3 by 3 block off the column are not either.
    assert!(!joints.test(cell(2, 8)) && !joints.test(cell(1, 1)));
    let key = typing.key(&state, cell(0, 9)).expect("a unit");
    assert_eq!(key.typ, Type::Guard);
    assert!(key.onj);
    assert_eq!((key.d, key.dj, key.cap), (2, 0, 0));
}

#[test]
fn a_unit_near_the_enemy_but_away_from_the_joints_is_a_reserve() {
    // (2, 8) is 3 steps from the enemy's row. The joint (0, 9) is within 3
    // steps of the enemy, but (2, 8) is 3 steps from it and not next to it, so
    // it is outside the guard zone.
    let state = line([0, 0], &[(0, 1, 1, 2, 8)]);
    let key = key_at(&state, 2, 8);
    assert_eq!(key.typ, Type::Reserve);
    assert_eq!((key.d, key.dj, key.onj), (3, 3, false));
    assert_eq!(key.own2, 0);
}

#[test]
fn units_far_back_are_free_or_shut_by_the_cells_next_to_them() {
    let state = line([0, 0], &[(0, 1, 1, 0, 6), (0, 2, 1, 0, 1), (0, 3, 3, 0, 5)]);
    let free = key_at(&state, 0, 6);
    assert_eq!(free.typ, Type::BackFree);
    assert_eq!((free.d, free.free1, free.adj, free.own2), (5, 1, 1, 1));
    // (0, 1): the HQ, (0, 2) and (1, 1) are red's, so no free cell is next to it.
    let shut = key_at(&state, 0, 1);
    assert_eq!(shut.typ, Type::BackShut);
    assert_eq!((shut.d, shut.free1, shut.dj), (10, 0, 0));
    assert!(shut.onj);
    // A level-3 unit, with the neighbour above it.
    let big = key_at(&state, 0, 5);
    assert_eq!(big.level, 3);
    assert_eq!(big.fields()[1], 1, "levels 2 and over are one");
    assert_eq!(free.fields()[1], 0);
}

#[test]
fn a_cell_without_a_unit_of_the_side_has_no_key() {
    let state = line([0, 0], &[(0, 1, 1, 0, 6), (1, 2, 1, 5, 11)]);
    let typing = Typing::new(&state);
    assert!(typing.key(&state, cell(0, 5)).is_none());
    assert!(typing.key(&state, cell(5, 11)).is_none(), "an enemy's unit");
    assert!(typing.key(&state, 5000).is_none(), "off the map");
}

#[cfg(feature = "analysis")]
#[test]
fn the_kinds_of_a_move_follow_the_distances_and_the_owners() {
    let state = line([0, 0], &[(0, 1, 1, 0, 6)]);
    let typing = Typing::new(&state);
    let kind = |from: (usize, usize), to: (usize, usize)| {
        typing.kind(&state, cell(from.0, from.1), cell(to.0, to.1))
    };
    assert_eq!(kind((2, 9), (2, 10)), Kind::ForwardN);
    assert_eq!(kind((0, 6), (0, 7)), Kind::ForwardO);
    assert_eq!(kind((0, 5), (1, 5)), Kind::SideN);
    assert_eq!(kind((1, 7), (0, 7)), Kind::SideO);
    assert_eq!(kind((1, 7), (1, 6)), Kind::BackN);
    assert_eq!(kind((0, 6), (0, 5)), Kind::BackO);
    assert_eq!(kind((0, 10), (0, 11)), Kind::Capture);
    assert_eq!(kind((0, 6), (0, 6)), Kind::Stay);
    // The names are the table's.
    let names: Vec<&str> = Kind::ALL.iter().map(|k| k.name()).collect();
    assert_eq!(
        names,
        [
            "capture",
            "forward-n",
            "forward-o",
            "side-n",
            "side-o",
            "back-n",
            "back-o",
            "stay"
        ]
    );
}

/// Red's HQ and blue's alone, far apart, so that no chain is in reach.
fn far(gold: [i64; 2]) -> State {
    board(&[], gold, &[])
}

#[test]
fn the_gold_bands_and_a_killing_turn() {
    let band = |gold| Typing::new(&far(gold)).gold_band();
    assert_eq!(band([0, 0]), Gold::Under40);
    assert_eq!(band([39, 0]), Gold::Under40);
    assert_eq!(band([40, 0]), Gold::Under80);
    assert_eq!(band([79, 0]), Gold::Under80);
    assert_eq!(band([80, 0]), Gold::Rich);
    assert_eq!(band([150, 0]), Gold::Rich);
    // Our gold pays a chain to the enemy HQ, 22 steps away: 220 gold or more.
    assert_eq!(band([1000, 0]), Gold::Kill);
    // Or theirs, with their income, pays one to ours.
    assert_eq!(band([0, 1000]), Gold::Kill);
    assert_eq!(band([10, 10]), Gold::Under40);
    // Next to the enemy's land the chain is cheaper, but still costs.
    assert_eq!(Typing::new(&line([1000, 0], &[])).gold_band(), Gold::Kill);
    assert_eq!(Typing::new(&line([0, 0], &[])).gold_band(), Gold::Under40);
}

#[cfg(feature = "analysis")]
#[test]
fn the_turn_bands() {
    let bands: Vec<&str> = [0, 4, 5, 6, 7, 8, 9, 12, 13, 20, 21, 90]
        .iter()
        .map(|&t| Band::of(t).name())
        .collect();
    assert_eq!(
        bands,
        [
            "5-6", "5-6", "5-6", "5-6", "7-8", "7-8", "9-12", "9-12", "13-20", "13-20", "21+",
            "21+"
        ]
    );
}

#[test]
fn a_side_with_no_joint_or_no_units_is_typed_without_a_panic() {
    // Red holds its HQ and one cell: no joint, and the enemy is 21 steps away.
    let state = board(&[(1, 0, b'O')], [0, 0], &[(0, 1, 1, 1, 0)]);
    let typing = Typing::new(&state);
    assert!(joints(&state, 0).is_empty());
    let key = typing.key(&state, cell(1, 0)).expect("a unit");
    assert_eq!((key.dj, key.onj, key.d), (FAR, false, 21));
    assert_eq!(key.fields()[9], 3);
    // No unit at all.
    let empty = far([0, 0]);
    let typing = Typing::new(&empty);
    assert!((0..144).all(|c| typing.key(&empty, c).is_none()));
}

#[test]
fn distances_to_nowhere_are_far() {
    // A pocket of one cell walled off by void cells has no path to the enemy.
    let walls = [(4, 5), (6, 5), (5, 4), (5, 6)].map(|(x, y)| (x, y, b'#'));
    let state = board(&walls, [0, 0], &[]);
    let typing = Typing::new(&state);
    assert_eq!(typing.distance(cell(11, 11)), 0);
    assert_eq!(typing.distance(cell(0, 0)), 22);
    assert_eq!(typing.distance(cell(5, 5)), FAR, "a pocket");
    assert_eq!(typing.distance(cell(4, 5)), FAR, "a void cell");
    assert_eq!(typing.distance(5000), FAR, "off the map");
    // Two cells with no path to the enemy are on a level.
    assert_eq!(typing.kind(&state, cell(5, 5), cell(5, 5)), Kind::Stay);
}

#[test]
fn codes_nest_by_level_and_clamp() {
    let state = line([0, 0], &[(0, 1, 1, 0, 10), (1, 2, 1, 1, 11)]);
    let mut key = key_at(&state, 0, 10);
    // Each level's code starts with the coarser level's.
    for depth in 1..LEVELS {
        let cut: u32 = WIDTHS[LENGTHS[depth - 1]..LENGTHS[depth]].iter().sum();
        assert_eq!(key.code(depth) >> cut, key.code(depth - 1), "L{depth}");
    }
    // Clamps: nothing beyond them changes a code.
    let base = key;
    key.level = 3;
    key.en2 = 7;
    key.own2 = 9;
    key.tw2 = 4;
    key.d = 50;
    key.dj = 99;
    key.free1 = 4;
    key.adj = 4;
    let mut other = key;
    other.level = 2;
    other.en2 = 2;
    other.own2 = 2;
    other.tw2 = 1;
    other.d = 4;
    other.dj = 3;
    other.free1 = 2;
    other.adj = 2;
    for depth in 0..LEVELS {
        assert_eq!(key.code(depth), other.code(depth));
    }
    assert_ne!(key.code(4), base.code(4));
    // Fields past a level's do not matter to it.
    let mut shifted = base;
    shifted.d = 3;
    assert_eq!(shifted.code(3), base.code(3));
    assert_ne!(shifted.code(4), base.code(4));
}

fn table_with(entries: &[(usize, u32, [u8; KINDS])]) -> Table {
    let mut table = Table::default();
    for &(depth, code, p) in entries {
        table.levels[depth].push((code, p));
    }
    for level in &mut table.levels {
        level.sort();
    }
    table
}

#[test]
fn the_prior_comes_from_the_finest_level_that_has_the_key() {
    let state = line([0, 0], &[(0, 1, 1, 0, 10)]);
    let key = key_at(&state, 0, 10);
    let (a, b, c) = (
        [255, 0, 0, 0, 0, 0, 0, 0],
        [0, 255, 0, 0, 0, 0, 0, 0],
        [0, 0, 255, 0, 0, 0, 0, 0],
    );
    assert_eq!(Table::default().prior(&key), [0; KINDS]);
    let only_l0 = table_with(&[(0, key.code(0), a)]);
    assert_eq!(only_l0.prior(&key), a);
    let more = table_with(&[
        (0, key.code(0), a),
        (1, key.code(1), b),
        (3, key.code(3), c),
    ]);
    assert_eq!(more.prior(&key), c);
    // A key that the finer levels do not have falls back.
    let mut other = key;
    other.cap = 0;
    other.en2 = 0;
    assert_eq!(more.prior(&other), b);
    other.band = Band::T21;
    assert_eq!(more.prior(&other), a);
}

#[cfg(feature = "analysis")]
#[test]
fn likeliest_orders_by_probability_then_kind() {
    assert!(likeliest(&[0; KINDS]).is_empty());
    let order = likeliest(&[10, 50, 50, 5, 0, 100, 0, 40]);
    let names: Vec<&str> = order.iter().map(|k| k.name()).collect();
    assert_eq!(
        names,
        [
            "back-n",
            "forward-n",
            "forward-o",
            "stay",
            "capture",
            "side-n",
            "side-o",
            "back-o"
        ]
    );
}

#[test]
fn the_embedded_table_decodes() {
    let table = Table::decode(table::TABLE).expect("the table written by types-table decodes");
    // A written table has all five levels, and entries that sum to 255.
    for level in &table.levels {
        for (_, p) in level {
            assert_eq!(p.iter().map(|&q| u32::from(q)).sum::<u32>(), 255);
        }
    }
    assert!(table.levels.iter().all(|l| !l.is_empty()));
    assert!(table::TABLE.chars().count() <= 8000);
}

#[cfg(feature = "analysis")]
#[test]
fn a_table_survives_its_bytes_and_the_packing() {
    let mut entries = Vec::new();
    for depth in 0..LEVELS {
        let mut code = 0u32;
        for i in 0..(30 + 7 * depth as u32) {
            code += 1 + (i * 37 + depth as u32 * 11) % 400;
            let mut p = [0u8; KINDS];
            let mut left = 255u32;
            for (k, q) in p.iter_mut().enumerate().take(KINDS - 1) {
                let v = (code.wrapping_mul(2_654_435_761).rotate_left(k as u32 * 3)) % (left + 1);
                *q = v as u8;
                left -= v;
            }
            p[KINDS - 1] = left as u8;
            entries.push((depth, code, p));
        }
    }
    let table = table_with(&entries);
    let bytes = crate::analysis::typing::to_bytes(&table);
    let back = Table::from_bytes(&bytes).expect("bytes");
    assert_eq!(back.levels, table.levels);
    let packed = crate::analysis::typing::pack(&bytes).expect("packed");
    assert!(packed.chars().all(|c| c as u32 >= 0x100));
    assert_eq!(cg_pack::unpack(&packed).expect("unpacked"), bytes);
    assert_eq!(packed, cg_packer::pack(&bytes), "the packer's own text");
    let decoded = Table::decode(&packed).expect("decoded");
    assert_eq!(decoded.levels, table.levels);
}

#[cfg(feature = "analysis")]
#[test]
fn bad_bytes_are_errors() {
    let table = table_with(&[
        (0, 3, [255, 0, 0, 0, 0, 0, 0, 0]),
        (1, 9, [100, 155, 0, 0, 0, 0, 0, 0]),
    ]);
    let bytes = crate::analysis::typing::to_bytes(&table);
    assert!(Table::from_bytes(&bytes).is_ok());
    for cut in 0..bytes.len() {
        assert!(Table::from_bytes(&bytes[..cut]).is_err(), "cut at {cut}");
    }
    let mut longer = bytes.clone();
    longer.push(0);
    assert!(Table::from_bytes(&longer).is_err());
    // Seven probabilities over 255; a count with no entries behind it.
    assert!(Table::from_bytes(&[1, 1, 200, 200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).is_err());
    assert!(Table::from_bytes(&[255, 255, 255, 255, 255]).is_err());
    assert!(Table::from_bytes(&[200, 1, 2]).is_err());
    assert!(Table::decode("not packed").is_err());
}
