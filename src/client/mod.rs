mod structs;

pub use structs::AniListResponseError;

use crate::builders::{AniListFragmentType, AniListResponse, QueryBuilder};
use crate::client::structs::{AniListResponseInternal, Request};
use crate::errors::AniListError;
use crate::models::Media;
use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde_json::{Map, Value};
use std::collections::HashSet;
use std::time::Duration;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(20);
const JSON_HEADER: HeaderValue = HeaderValue::from_static("application/json");

#[derive(Default, Clone)]
pub struct AniListClientBuilder<'a> {
    reqwest_client: Option<reqwest::Client>,
    anilist_token: Option<&'a str>,
    timeout: Option<Duration>,
}

impl<'a> AniListClientBuilder<'a> {
    pub fn with_reqwest_client(mut self, reqwest_client: reqwest::Client) -> Self {
        self.reqwest_client = Some(reqwest_client);

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

    pub fn build(self) -> Result<AniListClient<'a>, AniListError> {
        let reqwest_client = self.reqwest_client.unwrap_or_default();

        let anilist_token = self
            .anilist_token
            .ok_or(AniListError::MissingBuilderField("anilist_token"))?;

        let mut default_headers = HeaderMap::new();
        default_headers.insert(header::CONTENT_TYPE, JSON_HEADER);
        default_headers.insert(header::ACCEPT, JSON_HEADER);

        Ok(AniListClient {
            reqwest_client,
            anilist_token,
            timeout: self.timeout.unwrap_or(DEFAULT_TIMEOUT),
            default_headers,
        })
    }
}

#[derive(Clone)]
pub struct AniListClient<'a> {
    reqwest_client: reqwest::Client,
    anilist_token: &'a str,
    timeout: Duration,
    default_headers: HeaderMap,
}
impl<'a> AniListClient<'a> {
    pub fn builder() -> AniListClientBuilder<'a> {
        AniListClientBuilder::default()
    }

    pub async fn send_query(
        &self,
        query: impl QueryBuilder,
    ) -> Result<Option<AniListResponse>, AniListError> {
        Ok(self
            .send_queries(&[query])
            .await?
            .and_then(|values| values.into_iter().next()))
    }

    pub async fn send_queries(
        &self,
        queries: &[impl QueryBuilder],
    ) -> Result<Option<Vec<AniListResponse>>, AniListError> {
        let first = match queries.first() {
            Some(first) => first,
            None => return Ok(None),
        };

        if !queries
            .iter()
            .all(|q| q.root_operation_type() == first.root_operation_type())
        {
            return Err(AniListError::MixedOperationTypes);
        }

        let mut fragments = Vec::new();
        let mut variables = Map::new();
        let mut parameters = HashSet::new();
        let mut fragments_types = Vec::new();
        let mut access_token: Option<&str> = None;
        let mut need_auth = false;

        for (index, query) in queries.iter().enumerate() {
            let built_query = query.build(index);
            fragments.push(format!("q{index}: {}", built_query.field_fragment));

            for (key, value) in built_query.variables {
                variables.insert(key, value);
            }

            for parameter_definition in built_query.parameters_definitions {
                parameters.insert(parameter_definition);
            }

            fragments_types.push(query.get_fragment_type());

            if query.get_access_token().is_some() {
                access_token = query.get_access_token();
            }
            if query.need_auth() {
                need_auth = true;
            }
        }

        let query = format!(
            "{} ({}) {{ {} }}",
            first.root_operation_type(),
            parameters.into_iter().collect::<Vec<_>>().join(", "),
            fragments.join(" ")
        );

        self.request(&query, variables, &fragments_types, access_token, need_auth)
            .await
    }

    async fn request(
        &self,
        query: &str,
        variables: Map<String, Value>,
        fragments_types: &[AniListFragmentType],
        access_token: Option<&str>,
        need_auth: bool,
    ) -> Result<Option<Vec<AniListResponse>>, AniListError> {
        let mut headers = self.default_headers.clone();
        if let Some(access_token) = access_token {
            headers.insert("Authorization", format!("Bearer {access_token}").parse()?);
        }

        let mut body = self
            .reqwest_client
            .post("https://graphql.anilist.co/")
            .headers(headers)
            .timeout(self.timeout)
            .json(&Request { query, variables });
        if need_auth {
            body = body.bearer_auth(self.anilist_token);
        }

        let json: AniListResponseInternal<Map<String, Value>> = body.send().await?.json().await?;
        let mut data = match json.data {
            Some(data) => data,
            None => {
                return match json.errors {
                    Some(errors) => Err(AniListError::ApiErrors(errors)),
                    None => Err(AniListError::UnknownApiError),
                };
            }
        };
        data.sort_keys();

        Ok(Some(
            data.into_iter()
                .enumerate()
                .filter_map(|(i, (_, value))| match fragments_types[i] {
                    AniListFragmentType::Media => {
                        if let Ok(json) = serde_json::from_value::<Media>(value) {
                            Some(AniListResponse::Media(json))
                        } else {
                            None
                        }
                    }
                })
                .collect(),
        ))
    }
}
