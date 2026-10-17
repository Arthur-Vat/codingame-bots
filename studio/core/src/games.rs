//! The games the studio offers. A new game is added here, once: the server
//! and the WebAssembly program both register this list.

use studio_game::StudioGame;

/// The registered games, in the order the studio lists them.
pub fn registered() -> Vec<Box<dyn StudioGame>> {
    vec![
        Box::new(uttt_studio::Uttt),
        Box::new(ice_and_fire_studio::IceAndFire),
    ]
}

/// The game called `id` among `games`.
pub fn find<'a>(games: &'a [Box<dyn StudioGame>], id: &str) -> Option<&'a dyn StudioGame> {
    games
        .iter()
        .map(|game| game.as_ref())
        .find(|game| game.info().id == id)
}
