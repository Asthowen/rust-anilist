mod client;
mod errors;
pub mod models;
mod queries;

pub use self::client::{AniListClient, AniListClientBuilder};
pub use self::errors::GenericError;
