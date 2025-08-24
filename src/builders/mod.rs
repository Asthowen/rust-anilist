use crate::models::{Media, MediaList};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::{Display, Formatter};

pub mod fields;
mod mutations;
mod parameters_builder;
mod queries;

pub use mutations::save_media_list_entry::SaveMediaListEntryMutationBuilder;
pub(crate) use parameters_builder::ParametersBuilder;
pub use queries::media::MediaQueryBuilder;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AniListFragmentType {
    Media,
    MediaList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AniListResponse {
    Media(Box<Media>),
    MediaList(Box<MediaList>),
}

pub struct BuiltQuery {
    pub field_fragment: String,
    pub variables: Map<String, Value>,
    pub parameters_definitions: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RootOperationType {
    Query,
    Mutation,
    Subscription,
}

impl Display for RootOperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RootOperationType::Query => f.write_str("query"),
            RootOperationType::Mutation => f.write_str("mutation"),
            RootOperationType::Subscription => f.write_str("subscription"),
        }
    }
}

pub trait QueryBuilder {
    fn build(&self, index: usize) -> BuiltQuery;
    fn root_operation_type(&self) -> RootOperationType;
    fn get_fragment_type(&self) -> AniListFragmentType;
}

impl QueryBuilder for MediaQueryBuilder<'_> {
    fn build(&self, index: usize) -> BuiltQuery {
        self.build(index)
    }

    fn root_operation_type(&self) -> RootOperationType {
        RootOperationType::Query
    }

    fn get_fragment_type(&self) -> AniListFragmentType {
        AniListFragmentType::Media
    }
}

impl QueryBuilder for SaveMediaListEntryMutationBuilder<'_> {
    fn build(&self, index: usize) -> BuiltQuery {
        self.build(index)
    }

    fn root_operation_type(&self) -> RootOperationType {
        RootOperationType::Mutation
    }

    fn get_fragment_type(&self) -> AniListFragmentType {
        AniListFragmentType::MediaList
    }
}
