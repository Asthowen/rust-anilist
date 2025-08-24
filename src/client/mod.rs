mod structs;

pub use structs::AniListResponseError;

use crate::builders::{AniListFragmentType, AniListResponse, QueryBuilder};
use crate::client::structs::{AniListResponseInternal, Request};
use crate::errors::AniListError;
use crate::models::{Media, MediaList};
use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::collections::HashSet;
use std::time::Duration;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(20);
const JSON_HEADER: HeaderValue = HeaderValue::from_static("application/json");

#[derive(Default, Clone)]
pub struct AniListClientBuilder {
    reqwest_client: Option<reqwest::Client>,
    timeout: Option<Duration>,
}

impl AniListClientBuilder {
    pub fn with_reqwest_client(mut self, reqwest_client: reqwest::Client) -> Self {
        self.reqwest_client = Some(reqwest_client);

        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);

        self
    }

    pub fn build(self) -> Result<AniListClient, AniListError> {
        let reqwest_client = self.reqwest_client.unwrap_or_default();

        let mut default_headers = HeaderMap::new();
        default_headers.insert(header::CONTENT_TYPE, JSON_HEADER);
        default_headers.insert(header::ACCEPT, JSON_HEADER);

        Ok(AniListClient {
            reqwest_client,
            timeout: self.timeout.unwrap_or(DEFAULT_TIMEOUT),
            default_headers,
        })
    }
}

#[derive(Clone)]
pub struct AniListClient {
    reqwest_client: reqwest::Client,
    timeout: Duration,
    default_headers: HeaderMap,
}
impl AniListClient {
    pub fn builder() -> AniListClientBuilder {
        AniListClientBuilder::default()
    }

    pub async fn send_query(
        &self,
        query: impl QueryBuilder,
        access_token: Option<&str>,
    ) -> Result<Option<AniListResponse>, AniListError> {
        Ok(self
            .send_queries(&[query], access_token)
            .await?
            .and_then(|values| values.into_iter().next()))
    }

    pub async fn send_queries(
        &self,
        queries: &[impl QueryBuilder],
        access_token: Option<&str>,
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
        }

        let query = format!(
            "{} ({}) {{ {} }}",
            first.root_operation_type(),
            parameters.into_iter().collect::<Vec<_>>().join(", "),
            fragments.join(" ")
        );

        self.request(&query, variables, &fragments_types, access_token)
            .await
    }

    fn parse<T: DeserializeOwned>(
        value: Value,
        wrap: fn(Box<T>) -> AniListResponse,
    ) -> Option<AniListResponse> {
        serde_json::from_value::<T>(value)
            .ok()
            .map(|v| wrap(Box::new(v)))
    }

    async fn request(
        &self,
        query: &str,
        variables: Map<String, Value>,
        fragments_types: &[AniListFragmentType],
        access_token: Option<&str>,
    ) -> Result<Option<Vec<AniListResponse>>, AniListError> {
        let mut request = self
            .reqwest_client
            .post("https://graphql.anilist.co/")
            .headers(self.default_headers.clone())
            .timeout(self.timeout)
            .json(&Request { query, variables });
        if let Some(access_token) = access_token {
            request = request.bearer_auth(access_token);
        }

        let response = request.send().await?;
        let status_code = response.status();
        let json: AniListResponseInternal<Map<String, Value>> = response.json().await?;

        if let Some(errors) = json.errors {
            return Err(AniListError::ApiErrors(errors));
        }
        let mut data = json
            .data
            .ok_or_else(|| AniListError::UnknownApiError(status_code))?;
        data.sort_keys();

        Ok(Some(
            data.into_iter()
                .enumerate()
                .filter_map(|(i, (_, value))| match fragments_types[i] {
                    AniListFragmentType::Media => {
                        Self::parse::<Media>(value, AniListResponse::Media)
                    }
                    AniListFragmentType::MediaList => {
                        Self::parse::<MediaList>(value, AniListResponse::MediaList)
                    }
                })
                .collect(),
        ))
    }
}
