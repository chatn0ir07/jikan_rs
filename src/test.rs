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

#[test]
fn spike_spiegel_character_by_id() {
    let c: sync::Character = sync::Character::from(1).expect("Character");
    assert_eq!(c.name, "Spike Spiegel");
}

#[test]
fn user_endpoint() {
    let u: sync::User = sync::User::from("nekomata1037").expect("User");
    assert_eq!(u.username, "Nekomata1037");
}

#[test]
fn club_endpoint() {
    let u: sync::Club = sync::Club::from(1).expect("User");
    assert_eq!(u.title, "Cowboy Bebop");
}
