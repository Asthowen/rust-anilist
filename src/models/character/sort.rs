use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CharacterSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "ROLE")]
    Role,
    #[serde(rename = "ROLE_DESC")]
    RoleDesc,
    #[serde(rename = "SEARCH_MATCH")]
    SearchMatch,
    #[serde(rename = "FAVOURITES")]
    Favourites,
    #[serde(rename = "FAVOURITES_DESC")]
    FavouritesDesc,
    #[serde(rename = "Relevance")]
    Relevance,
}
