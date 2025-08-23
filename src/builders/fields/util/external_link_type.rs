#[derive(Copy, Clone)]
pub enum ExternalLinksField {
    Id,
    Url,
    Site,
    SiteId,
    Type,
    Language,
    Color,
    Icon,
}

impl ExternalLinksField {
    pub const fn all() -> &'static [ExternalLinksField] {
        &[
            ExternalLinksField::Id,
            ExternalLinksField::Url,
            ExternalLinksField::Site,
            ExternalLinksField::SiteId,
            ExternalLinksField::Type,
            ExternalLinksField::Language,
            ExternalLinksField::Color,
            ExternalLinksField::Icon,
        ]
    }
}

impl From<ExternalLinksField> for String {
    fn from(value: ExternalLinksField) -> Self {
        match value {
            ExternalLinksField::Id => "id".to_owned(),
            ExternalLinksField::Url => "url".to_owned(),
            ExternalLinksField::Site => "site".to_owned(),
            ExternalLinksField::SiteId => "siteId".to_owned(),
            ExternalLinksField::Type => "type".to_owned(),
            ExternalLinksField::Language => "language".to_owned(),
            ExternalLinksField::Color => "color".to_owned(),
            ExternalLinksField::Icon => "icon".to_owned(),
        }
    }
}
