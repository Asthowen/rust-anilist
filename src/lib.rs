pub mod builders;
pub mod models;

mod client;
mod errors;

pub use self::client::{AniListClient, AniListClientBuilder};
pub use self::errors::AniListError;
