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

impl From<ExternalLinksField> for &'static str {
    fn from(value: ExternalLinksField) -> Self {
        match value {
            ExternalLinksField::Id => "id",
            ExternalLinksField::Url => "url",
            ExternalLinksField::Site => "site",
            ExternalLinksField::SiteId => "siteId",
            ExternalLinksField::Type => "type",
            ExternalLinksField::Language => "language",
            ExternalLinksField::Color => "color",
            ExternalLinksField::Icon => "icon",
        }
    }
}
