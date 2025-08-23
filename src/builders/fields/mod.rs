mod airing;
mod character;
mod media;
mod staff;
mod util;

pub use airing::schedule::AiringScheduleField;
pub use character::CharacterField;
pub use character::connection::CharacterConnectionField;
pub use character::edge::CharacterEdgeField;
pub use character::image::CharacterImageField;
pub use character::name::CharacterNameField;
pub use media::MediaField;
pub use media::connection::MediaConnectionField;
pub use media::cover_image::MediaCoverImageField;
pub use media::edge::MediaEdgeField;
pub use media::rank::MediaRankField;
pub use media::streaming_episode::MediaStreamingEpisodesField;
pub use media::tag::MediaTagField;
pub use media::title::MediaTitleField;
pub use media::trailer::MediaTrailerField;
pub use staff::StaffField;
pub use staff::connection::StaffConnectionField;
pub use staff::edge::StaffEdgeField;
pub use staff::image::StaffImageField;
pub use staff::name::StaffNameField;
pub use staff::role_type::StaffRoleTypeField;
pub use util::external_link_type::ExternalLinksField;
pub use util::page_info::PageInfoField;

pub(crate) trait JoinFields {
    fn join_fields(&self) -> String;
}

impl<T> JoinFields for [T]
where
    T: Copy + Into<String>,
{
    fn join_fields(&self) -> String {
        self.iter()
            .map(|&f| f.into())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
