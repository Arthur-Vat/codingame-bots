//! The leagues, whose rules grow from Wood 3 to Bronze (RULES.md, "Leagues").

/// The league whose rules are played. It has the same variants and methods as
/// the referee's, which a bot cannot depend on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum League {
    Wood3,
    Wood2,
    Wood1,
    #[default]
    Bronze,
}

impl League {
    /// The highest unit level that can be trained.
    pub fn max_level(self) -> u8 {
        match self {
            League::Wood3 => 1,
            _ => 3,
        }
    }

    /// Whether mines can be built.
    pub fn allows_mines(self) -> bool {
        matches!(self, League::Wood1 | League::Bronze)
    }

    /// Whether towers can be built.
    pub fn allows_towers(self) -> bool {
        self == League::Bronze
    }

    /// Whether the map has mine spots.
    pub fn has_mine_spots(self) -> bool {
        matches!(self, League::Wood1 | League::Bronze)
    }
}
