use super::*;


#[test]
fn violet_evergarden_from_id() {
    let a: sync::Anime = sync::Anime::from(33352).expect("Violet Evergarden");
    assert_eq!(a.title, "Violet Evergarden");
}

#[test]
fn monster_from_id() {
    let m: sync::Manga = sync::Manga::from(1).expect("Monster");
    assert_eq!(m.title, "Monster");

}