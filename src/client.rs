use crate::errors::GenericError;
use crate::models::{Anime, Character, Manga};
use crate::queries;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Clone)]
pub struct AniListClientBuilder<'a> {
    reqwest_client: Option<reqwest::Client>,
    anilist_token: Option<&'a str>,
    timeout: Option<Duration>,
}

impl<'a> AniListClientBuilder<'a> {
    pub fn builder() -> Self {
        Self {
            reqwest_client: None,
            anilist_token: None,
            timeout: None,
        }
    }

    pub fn with_reqwest_client(mut self, reqwest_client: reqwest::Client) -> Self {
        self.reqwest_client = Some(reqwest_client);

        self
    }

    pub fn with_initialized_reqwest_client(mut self) -> Self {
        self.reqwest_client = Some(reqwest::Client::new());

        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);

        self
    }

    pub fn with_anilist_token(mut self, anilist_token: &'a str) -> Self {
        self.anilist_token = Some(anilist_token);

        self
    }

    pub fn build(&self) -> Result<AniListClient<'a>, GenericError> {
        if self.reqwest_client.is_none() {
            return Err(GenericError(
                "You have not filled in all the required elements in the builder: reqwest_client"
                    .to_owned(),
            ));
        }
        if self.anilist_token.is_none() {
            return Err(GenericError(
                "You have not filled in all the required elements in the builder: anilist_token"
                    .to_owned(),
            ));
        }
        if self.anilist_token.is_none() {
            return Err(GenericError(
                "You have not filled in all the required elements in the builder: anilist_token"
                    .to_owned(),
            ));
        }

        Ok(AniListClient {
            reqwest_client: self.reqwest_client.clone().unwrap(),
            anilist_token: self.anilist_token.unwrap(),
            timeout: self.timeout.unwrap_or(Duration::from_secs(20)),
        })
    }
}

#[derive(Clone)]
pub struct AniListClient<'a> {
    reqwest_client: reqwest::Client,
    anilist_token: &'a str,
    timeout: Duration,
}
impl<'a> AniListClient<'a> {
    pub async fn get_anime(&self, variables: Value) -> Option<Anime> {
        let data: Value = self
            .request("anime", "get", variables, false, None)
            .await
            .unwrap();
        let mut anime: Anime = Anime::parse(&data["data"]["Media"]);
        anime.is_full_loaded = true;

        Some(anime)
    }

    pub async fn get_manga(&self, variables: Value) -> Option<Manga> {
        let data = self
            .request("manga", "get", variables, false, None)
            .await
            .unwrap();
        let mut manga = Manga::parse(&data["data"]["Media"]);
        manga.is_full_loaded = true;

        Some(manga)
    }

    pub async fn get_character(&self, variables: Value) -> Option<Character> {
        let data = self
            .request("character", "get", variables, false, None)
            .await
            .unwrap();
        let mut character = Character::parse(&data["data"]["Character"]);
        character.is_full_loaded = true;

        Some(character)
    }

    pub async fn set_progress(
        &self,
        new_chapter: i64,
        media_id: i64,
        access_token: &str,
    ) -> Result<Value, GenericError> {
        self.request(
            "progress",
            "set",
            json!({"progress": new_chapter, "mediaId": media_id, "hidden": true}),
            false,
            Some(access_token),
        )
        .await
    }

    pub async fn set_increment_progress(
        &self,
        progress_start: i64,
        progress_end: i64,
        media_id: i64,
        access_token: &str,
    ) -> Result<Value, GenericError> {
        self
            .request(
                "progress_increment",
                "set",
                json!({"progress_start": progress_start, "progress_end": progress_end, "mediaId": media_id, "hidden": true}),
                false,
                Some(access_token),
            )
            .await
    }

    async fn request(
        &self,
        media_type: &str,
        action: &str,
        variables: Value,
        need_auth: bool,
        access_token: Option<&str>,
    ) -> Result<Value, GenericError> {
        let query: &str = if let Some(query) = AniListClient::get_query(media_type, action) {
            query
        } else {
            return Err(GenericError(
                "The type of query entered is not available.".to_owned(),
            ));
        };
        let json: Value = json!({"query": query, "variables": variables});
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        if let Some(access_token) = access_token {
            let header_value = format!("Bearer {}", access_token).parse().unwrap();
            headers.insert("Authorization", header_value);
        }

        let mut body = self
            .reqwest_client
            .post("https://graphql.anilist.co/")
            .headers(headers)
            .timeout(self.timeout)
            .body(json.to_string());
        if need_auth {
            body = body.bearer_auth(self.anilist_token);
        }

        let response: String = body.send().await?.text().await?;
        let result = serde_json::from_str(&response);
        let result_value: Value = if let Ok(result_value) = result {
            result_value
        } else if let Err(result_error) = result {
            return Err(GenericError(format!("serde_json error: {}", result_error)));
        } else {
            return Err(GenericError("Unknown error".to_owned()));
        };

        Ok(result_value)
    }

    pub fn get_query(media_type: &str, action: &str) -> Option<&'a str> {
        const MEDIA_TYPES: [&str; 9] = [
            "anime",
            "manga",
            "character",
            "user",
            "person",
            "studio",
            "progress",
            "mediasids",
            "progress_increment",
        ];
        if !MEDIA_TYPES.contains(&media_type) {
            return None;
        }

        match (action, media_type) {
            ("get", "anime") => Some(queries::get_anime::GET_ANIME),
            ("get", "character") => Some(queries::get_character::GET_CHARACTER),
            ("get", "manga") => Some(queries::get_manga::GET_MANGA),
            ("get", "person") => Some(queries::get_person::GET_PERSON),
            ("get", "mediasids") => Some(queries::get_mediasids::GET_MEDIAS_IDS),
            ("set", "progress") => Some(queries::set_progress::SET_PROGRESS),
            ("set", "progress_increment") => Some(queries::set_progress::SET_PROGRESS_INCREMENT),
            _ => None,
        }
    }
}
