use crate::builders::fields::PageInfoField;
use crate::builders::fields::media::edge::MediaEdgeField;

#[derive(Copy, Clone)]
pub enum MediaConnectionField<'a> {
    Edges(&'a [MediaEdgeField<'a>]),
    PageInfo(&'a [PageInfoField]),
}

impl MediaConnectionField<'_> {
    pub const fn all() -> &'static [MediaConnectionField<'static>] {
        const ALL: &[MediaConnectionField<'static>] = &[
            MediaConnectionField::Edges(MediaEdgeField::all()),
            MediaConnectionField::PageInfo(PageInfoField::all()),
        ];
        ALL
    }
}

impl From<MediaConnectionField<'_>> for String {
    fn from(value: MediaConnectionField) -> Self {
        match value {
            MediaConnectionField::Edges(fields) => format!(
                "edges {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            MediaConnectionField::PageInfo(fields) => format!(
                "pageInfo {{ {} }}",
                fields
                    .iter()
                    .map(|&field| field.into())
                    .collect::<Vec<&str>>()
                    .join(" ")
            ),
        }
    }
}
