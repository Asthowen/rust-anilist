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
pub use media::list::MediaListField;
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
    fn join_fields_index(&self, index: usize) -> Vec<String>;
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

    fn join_fields_index(&self, index: usize) -> Vec<String> {
        let index_string = index.to_string();
        self.iter()
            .map(|&field| {
                let field_string: String = field.into();
                field_string.replace("%%index%%", &index_string)
            })
            .collect::<Vec<String>>()
    }
}
