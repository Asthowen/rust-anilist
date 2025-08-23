mod airing;
mod character;
mod graphql;
mod media;
mod staff;
mod util;

pub use character::Character;
pub use character::connection::CharacterConnection;
pub use character::edge::CharacterEdge;
pub use character::image::CharacterImage;
pub use character::name::CharacterName;
pub use character::role::CharacterRole;
pub use character::sort::CharacterSort;
pub use media::Media;
pub use media::cover_image::MediaCoverImage;
pub use media::external_link::MediaExternalLink;
pub use media::format::MediaFormat;
pub use media::mtype::MediaType;
pub use media::rank::MediaRank;
pub use media::rank_type::MediaRankType;
pub use media::season::MediaSeason;
pub use media::source::MediaSource;
pub use media::status::MediaStatus;
pub use media::tag::MediaTag;
pub use media::title::MediaTitle;
pub use media::trailer::MediaTrailer;
pub use staff::Staff;
pub use staff::connection::StaffConnection;
pub use staff::edge::StaffEdge;
pub use staff::image::StaffImage;
pub use staff::name::StaffName;
pub use staff::role_type::StaffRoleType;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaResponse {
    #[serde(rename = "Media")]
    pub media: Option<Media>,
}
