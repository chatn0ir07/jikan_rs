
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

}




#[cfg(test)]
mod test;


