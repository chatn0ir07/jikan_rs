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


#[test]
fn season_spring_2020() {
    let a: Vec<sync::Anime> = sync::get_season(2020, "spring").expect("2020's spring season");

}

#[test]
fn future_season() {
    let a: Vec<sync::Anime> = sync::get_future_season().expect("Array");
}

#[test]
fn tomokazu_seki_person_by_id() {
    let p: sync::Person = sync::Person::from(1).expect("Person");
    assert_eq!(p.name, "Tomokazu Seki");
}