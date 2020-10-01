use super::*;


#[test]
fn violet_evergarden_from_id() {
    let a: sync::Anime = sync::Anime::from(33352).expect("Violet Evergarden");
    assert_eq!(a.title, "Violet Evergarden");
}