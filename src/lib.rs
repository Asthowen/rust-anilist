pub mod builders;
mod client;
mod errors;
pub mod models;

pub use self::errors::AniListError;
pub use crate::client::{AniListClient, AniListClientBuilder};
