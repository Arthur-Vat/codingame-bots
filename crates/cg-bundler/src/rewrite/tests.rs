use super::*;

fn libraries() -> Vec<String> {
    vec!["cg_core".to_string(), "uttt_engine".to_string()]
}

#[test]
fn bot_code_reaches_libraries_from_the_bundle_root() {
    let bot =
        "use cg_core::rng::Rng;\nuse std::io;\nfn f() { let x = uttt_engine::Board::new(); }\n";
    assert_eq!(
        rewrite(bot, None, &libraries()),
        "use crate::cg_core::rng::Rng;\nuse std::io;\nfn f() { let x = crate::uttt_engine::Board::new(); }\n"
    );
}

#[test]
fn library_code_refers_to_itself_through_its_module() {
    let library = "use crate::rng::Rng;\npub(crate) fn f() {}\npub(in crate::a) fn g() {}\nmacro_rules! m { () => { $crate::rng::Rng::new(1) } }\n";
    assert_eq!(
        rewrite(library, Some("cg_core"), &libraries()),
        "use crate::cg_core::rng::Rng;\npub(crate) fn f() {}\npub(in crate::cg_core::a) fn g() {}\nmacro_rules! m { () => { $crate::cg_core::rng::Rng::new(1) } }\n"
    );
}

#[test]
fn a_library_can_use_another_library() {
    let engine = "use cg_core::rng::Rng;\nuse crate::board::Board;\n";
    assert_eq!(
        rewrite(engine, Some("uttt_engine"), &libraries()),
        "use crate::cg_core::rng::Rng;\nuse crate::uttt_engine::board::Board;\n"
    );
}

#[test]
fn leaves_names_that_only_contain_a_library_name_alone() {
    let text = "my_cg_core::x; std::cg_core::y; {cg_core::z}\n";
    assert_eq!(
        rewrite(text, None, &libraries()),
        "my_cg_core::x; std::cg_core::y; {crate::cg_core::z}\n"
    );
}

#[test]
fn removes_extern_crate_lines_of_libraries() {
    let text = "extern crate cg_core;\nextern crate alloc;\nfn main() {}\n";
    assert_eq!(
        rewrite(text, None, &libraries()),
        "extern crate alloc;\nfn main() {}\n"
    );
}
