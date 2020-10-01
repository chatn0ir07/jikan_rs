
pub mod sync {
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Anime {
        pub url: String,
        pub image_url: String,
        pub trailer_url: Option<String>,
        pub title: String,
        pub airing: bool,
        pub synopsis: Option<String>,
        pub show_type: Option<String>,
        pub episodes: i32,
        pub score: f32,
        pub start_date: Option<String>,
        pub end_date: Option<String>,
        #[serde(rename = "rated")]
        pub rating: Option<String>
    }

    impl Anime {
        pub fn from<T: Into<i32>>(id: T) -> Option<Anime> {
            let req = reqwest::blocking::get(format!("https://api.jikan.moe/v3/anime/{}", id.into()).as_str());
            match req {
                Ok(x) => {
                    let output = &x.text().unwrap();
                    if let Ok(json) = serde_json::from_str(output) {
                        Some(json)
                    }else{
                        None
                    }
                },
                _ => {
                    None
                }
            }
        }
    }

    impl std::fmt::Debug for Anime {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(&serde_json::to_string(self).unwrap())
        }
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Manga {
        pub url: String,
        pub title: String,
        pub title_english: String,
        pub title_japanese: String,
        pub image_url: String,
        pub r#type: String,
        pub volumes: u32,
        pub chapters: u32,
        pub synopsis: Option<String>,
        pub score: f32,
        pub scored_by: usize,
    }

    impl Manga {
        pub fn from<T: Into<i32>>(id: T) -> Option<Manga> {
            let req = reqwest::blocking::get(format!("https://api.jikan.moe/v3/manga/{}", id.into()).as_str());
            match req {
                Ok(x) => {
                    let output = &x.text().unwrap();
                    if let Ok(json) = serde_json::from_str(output) {
                        Some(json)
                    }else{
                        None
                    }
                },
                _ => {
                    None
                }
            }
        }
    }

    impl std::fmt::Debug for Manga {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(&serde_json::to_string(self).unwrap())
        }
    }


    pub fn get_season(year: u64, season: impl Into<String>) -> Result<Vec<Anime>, ()> {
        let req = reqwest::blocking::get(format!("https://api.jikan.moe/v3/season/{}/{}", year, season.into()).as_str());
        match req {
            Ok(x) => {
                let output = &x.text().unwrap();
                #[derive(serde::Serialize, serde::Deserialize)]
                struct Season {
                    anime: Vec<Anime>
                }

                match serde_json::from_str::<Season>(output){
                    Ok(json) => {
                        Ok(json.anime)
                    },
                    Err(x) => {
                        Err(())
                    }
                }
            },
            _ => Err(())
        }

    }

    pub fn get_future_season() -> Result<Vec<Anime>, ()> {
        let req = reqwest::blocking::get(format!("https://api.jikan.moe/v3/season/later").as_str());
        match req {
            Ok(x) => {
                let output = &x.text().unwrap();
                #[derive(serde::Serialize, serde::Deserialize)]
                struct Season {
                    anime: Vec<Anime>
                }

                match serde_json::from_str::<Season>(output){
                    Ok(json) => {
                        Ok(json.anime)
                    },
                    Err(x) => {
                        Err(())
                    }
                }
            },
            _ => Err(())
        }

    }




    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Person {
        pub url: String,
        pub image_url: Vec<String>,
        pub website_url: Option<String>,
        pub name: String,
        pub given_name: String,
        pub family_name: String,
        pub birthday: String
    }

    impl Person {
        pub fn from<T: Into<i32>>(id: T) -> Option<Person> {
            let url = format!("https://api.jikan.moe/v3/person/{}", id.into());
            let req = reqwest::blocking::get(&url);
            match req {
                Ok(x) => {
                    let output = &x.text().unwrap();
                    if let Ok(json) = serde_json::from_str(output) {
                        Some(json)
                    }else{
                        None
                    }
                },
                _ => {
                    None
                }
            }
        }
    }

    impl std::fmt::Debug for Person {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(&serde_json::to_string(self).unwrap())
        }
    }


    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Character {
        pub url: String,
        pub name: String,
        pub nicknames: Vec<String>,
        pub about: String
    }

    impl Character {
        pub fn from<T: Into<i32>>(id: T) -> Option<Character> {
            let url = format!("https://api.jikan.moe/v3/character/{}", id.into());
            let req = reqwest::blocking::get(&url);
            match req {
                Ok(x) => {
                    let output = &x.text().unwrap();
                    if let Ok(json) = serde_json::from_str(output) {
                        Some(json)
                    }else{
                        None
                    }
                },
                _ => {
                    None
                }
            }
        }
    }

    impl std::fmt::Debug for Character {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(&serde_json::to_string(self).unwrap())
        }
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct User {
        pub user_id: usize,
        pub username: String,
        pub url: String,
        pub image_url: String,
        pub last_online: String,
        pub gender: String,
        pub birthday: String,
        pub location: String,
        pub joined: String,
    }

    impl User {
        pub fn from<T: Into<String>>(name: T) -> Option<User> {
            let url = format!("https://api.jikan.moe/v3/user/{}", name.into());
            let req = reqwest::blocking::get(&url);
            match req {
                Ok(x) => {
                    let output = &x.text().unwrap();
                    if let Ok(json) = serde_json::from_str(output) {
                        Some(json)
                    }else{
                        None
                    }
                },
                _ => {
                    None
                }
            }
        }
    }

    impl std::fmt::Debug for User {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str(&serde_json::to_string(self).unwrap())
        }
    }

}




#[cfg(test)]
mod test;


