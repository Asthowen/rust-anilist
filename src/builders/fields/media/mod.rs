pub(crate) mod connection;
pub(crate) mod cover_image;
pub(crate) mod edge;
pub(crate) mod rank;
pub(crate) mod streaming_episode;
pub(crate) mod tag;
pub(crate) mod title;
pub(crate) mod trailer;

use crate::builders::fields::airing::schedule::AiringScheduleField;
use crate::builders::fields::media::connection::MediaConnectionField;
use crate::builders::fields::media::streaming_episode::MediaStreamingEpisodesField;
use crate::builders::fields::{
    CharacterConnectionField, CharacterEdgeField, ExternalLinksField, MediaCoverImageField,
    MediaRankField, MediaTagField, MediaTitleField, MediaTrailerField, PageInfoField,
    StaffConnectionField,
};

#[derive(Copy, Clone)]
pub enum MediaField<'a> {
    Id,
    IdMal,
    Title(&'a [MediaTitleField]),
    Type,
    Format,
    Status,
    Description,
    StartDate,
    EndDate,
    Season,
    SeasonYear,
    SeasonInt,
    Episodes,
    Duration,
    Chapters,
    Volumes,
    CountryOfOrigin,
    IsLicensed,
    Source,
    Hashtag,
    Trailer(&'a [MediaTrailerField]),
    UpdatedAt,
    CoverImage(&'a [MediaCoverImageField]),
    BannerImage,
    Genres,
    Synonyms,
    AverageScore,
    MeanScore,
    Popularity,
    IsLocked,
    Trending,
    Favourites,
    Tags(&'a [MediaTagField]),
    Relations(&'a [MediaConnectionField<'a>]),
    Characters(&'a [CharacterConnectionField<'a>]),
    Staff(&'a [StaffConnectionField<'a>]),
    IsFavourite,
    IsFavouriteBlocked,
    IsAdult,
    NextAiringEpisode(&'a [AiringScheduleField]),
    ExternalLinks(&'a [ExternalLinksField]),
    StreamingEpisodes(&'a [MediaStreamingEpisodesField]),
    Rankings(&'a [MediaRankField]),
    SiteUrl,
    AutoCreateForumThread,
    IsRecommendationBlocked,
    IsReviewBlocked,
    ModNotes,
}

impl MediaField<'_> {
    pub const fn all() -> &'static [MediaField<'static>] {
        const ALL: &[MediaField<'static>] = &[
            MediaField::Id,
            MediaField::IdMal,
            MediaField::Title(MediaTitleField::all()),
            MediaField::Type,
            MediaField::Format,
            MediaField::Status,
            MediaField::Description,
            MediaField::StartDate,
            MediaField::EndDate,
            MediaField::Season,
            MediaField::SeasonYear,
            MediaField::SeasonInt,
            MediaField::Episodes,
            MediaField::Duration,
            MediaField::Chapters,
            MediaField::Volumes,
            MediaField::CountryOfOrigin,
            MediaField::IsLicensed,
            MediaField::Source,
            MediaField::Hashtag,
            MediaField::Trailer(MediaTrailerField::all()),
            MediaField::UpdatedAt,
            MediaField::CoverImage(MediaCoverImageField::all()),
            MediaField::BannerImage,
            MediaField::Genres,
            MediaField::Synonyms,
            MediaField::AverageScore,
            MediaField::MeanScore,
            MediaField::Popularity,
            MediaField::IsLocked,
            MediaField::Trending,
            MediaField::Favourites,
            MediaField::Tags(MediaTagField::all()),
            MediaField::Relations(MediaConnectionField::all()),
            MediaField::Characters(&[
                CharacterConnectionField::Edges(CharacterEdgeField::all_without_media()),
                CharacterConnectionField::PageInfo(PageInfoField::all()),
            ]),
            MediaField::Staff(StaffConnectionField::all()),
            MediaField::IsFavourite,
            MediaField::IsFavouriteBlocked,
            MediaField::IsAdult,
            MediaField::NextAiringEpisode(AiringScheduleField::all()),
            MediaField::ExternalLinks(ExternalLinksField::all()),
            MediaField::StreamingEpisodes(MediaStreamingEpisodesField::all()),
            MediaField::Rankings(MediaRankField::all()),
            MediaField::SiteUrl,
            MediaField::AutoCreateForumThread,
            MediaField::IsRecommendationBlocked,
            MediaField::IsReviewBlocked,
            MediaField::ModNotes,
        ];

        ALL
    }
}

impl From<MediaField<'_>> for String {
    fn from(value: MediaField) -> Self {
        match value {
            MediaField::Id => "id".to_owned(),
            MediaField::IdMal => "idMal".to_owned(),
            MediaField::Title(fields) => format!(
                "title {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            MediaField::Type => "type".to_owned(),
            MediaField::Format => "format".to_owned(),
            MediaField::Status => "status(version: 2)".to_owned(),
            MediaField::Description => "description(asHtml: $htmlDescription%%index%%)".to_owned(),
            MediaField::StartDate => "startDate { year month day }".to_owned(),
            MediaField::EndDate => "endDate { year month day }".to_owned(),
            MediaField::Season => "season".to_owned(),
            MediaField::SeasonYear => "seasonYear".to_owned(),
            MediaField::SeasonInt => "seasonInt".to_owned(),
            MediaField::Episodes => "episodes".to_owned(),
            MediaField::Duration => "duration".to_owned(),
            MediaField::Chapters => "chapters".to_owned(),
            MediaField::Volumes => "volumes".to_owned(),
            MediaField::CountryOfOrigin => "countryOfOrigin".to_owned(),
            MediaField::IsLicensed => "isLicensed".to_owned(),
            MediaField::Source => "source(version: 3)".to_owned(),
            MediaField::Hashtag => "hashtag".to_owned(),
            MediaField::Trailer(fields) => format!(
                "trailer {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            MediaField::UpdatedAt => "updatedAt".to_owned(),
            MediaField::CoverImage(fields) => format!(
                "coverImage {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            MediaField::BannerImage => "bannerImage".to_owned(),
            MediaField::Genres => "genres".to_owned(),
            MediaField::Synonyms => "synonyms".to_owned(),
            MediaField::AverageScore => "averageScore".to_owned(),
            MediaField::MeanScore => "meanScore".to_owned(),
            MediaField::Popularity => "popularity".to_owned(),
            MediaField::IsLocked => "isLocked".to_owned(),
            MediaField::Trending => "trending".to_owned(),
            MediaField::Favourites => "favourites".to_owned(),
            MediaField::Tags(fields) => format!(
                "tags {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            MediaField::Relations(fields) => format!(
                "relations {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            MediaField::Characters(fields) => format!(
                "characters {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            MediaField::Staff(fields) => format!(
                "staff {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            MediaField::IsFavourite => "isFavourite".to_owned(),
            MediaField::IsFavouriteBlocked => "isFavouriteBlocked".to_owned(),
            MediaField::IsAdult => "isAdult".to_owned(),
            MediaField::NextAiringEpisode(fields) => format!(
                "nextAiringEpisode {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            MediaField::ExternalLinks(fields) => format!(
                "externalLinks {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            MediaField::StreamingEpisodes(fields) => format!(
                "streamingEpisodes {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            MediaField::Rankings(fields) => format!(
                "rankings {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
            MediaField::SiteUrl => "siteUrl".to_owned(),
            MediaField::AutoCreateForumThread => "autoCreateForumThread".to_owned(),
            MediaField::IsRecommendationBlocked => "isRecommendationBlocked".to_owned(),
            MediaField::IsReviewBlocked => "isReviewBlocked".to_owned(),
            MediaField::ModNotes => "modNotes".to_owned(),
        }
    }
}
