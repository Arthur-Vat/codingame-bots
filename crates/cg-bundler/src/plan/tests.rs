use super::*;

#[test]
fn plans_the_random_bot_of_this_workspace() {
    let plan = plan("uttt-bot-random", None).unwrap();
    assert_eq!(plan.package, "uttt-bot-random");
    assert_eq!(plan.bot.name, "uttt_bot_random");
    assert!(plan
        .bot
        .root
        .ends_with("games/uttt/bots/random/src/main.rs"));
    let names: Vec<&str> = plan.libraries.iter().map(|lib| lib.name.as_str()).collect();
    assert_eq!(names, ["cg_core"]);
    assert!(plan.libraries[0]
        .root
        .ends_with("crates/cg-core/src/lib.rs"));
}

#[test]
fn refuses_packages_that_are_not_bots_or_use_registry_crates() {
    let error = plan("cg-core", None).unwrap_err();
    assert!(error.contains("exactly one binary"), "{error}");
    // The arena uses clap from crates.io.
    let error = plan("uttt-arena", None).unwrap_err();
    assert!(error.contains("from a registry"), "{error}");
    let error = plan("no-such-package", None).unwrap_err();
    assert!(error.contains("no package"), "{error}");
}
