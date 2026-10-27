mod airing;
mod character;
mod media;
mod staff;
mod util;

pub use self::airing::schedule::AiringScheduleField;
pub use self::character::CharacterField;
pub use self::character::connection::CharacterConnectionField;
pub use self::character::edge::CharacterEdgeField;
pub use self::character::image::CharacterImageField;
pub use self::character::name::CharacterNameField;
pub use self::media::MediaField;
pub use self::media::connection::MediaConnectionField;
pub use self::media::cover_image::MediaCoverImageField;
pub use self::media::edge::MediaEdgeField;
pub use self::media::list::MediaListField;
pub use self::media::rank::MediaRankField;
pub use self::media::streaming_episode::MediaStreamingEpisodesField;
pub use self::media::tag::MediaTagField;
pub use self::media::title::MediaTitleField;
pub use self::media::trailer::MediaTrailerField;
pub use self::staff::StaffField;
pub use self::staff::connection::StaffConnectionField;
pub use self::staff::edge::StaffEdgeField;
pub use self::staff::image::StaffImageField;
pub use self::staff::name::StaffNameField;
pub use self::staff::role_type::StaffRoleTypeField;
pub use self::util::external_link_type::ExternalLinksField;
pub use self::util::page_info::PageInfoField;

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
