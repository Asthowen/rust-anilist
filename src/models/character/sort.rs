use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CharacterSort {
    #[serde(alias = "ID")]
    Id,
    #[serde(alias = "ID_DESC")]
    IdDesc,
    #[serde(alias = "ROLE")]
    Role,
    #[serde(alias = "ROLE_DESC")]
    RoleDesc,
    #[serde(alias = "SEARCH_MATCH")]
    SearchMatch,
    #[serde(alias = "FAVOURITES")]
    Favourites,
    #[serde(alias = "FAVOURITES_DESC")]
    FavouritesDesc,
    #[serde(alias = "Relevance")]
    Relevance,
}
