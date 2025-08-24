#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PageInfoField {
    Total,
    PerPage,
    CurrentPage,
    LastPage,
    HasNextPage,
}

impl PageInfoField {
    pub const fn all() -> &'static [PageInfoField] {
        &[
            PageInfoField::Total,
            PageInfoField::PerPage,
            PageInfoField::CurrentPage,
            PageInfoField::LastPage,
            PageInfoField::HasNextPage,
        ]
    }
}

impl From<PageInfoField> for String {
    fn from(value: PageInfoField) -> Self {
        match value {
            PageInfoField::Total => "total".to_owned(),
            PageInfoField::PerPage => "perPage".to_owned(),
            PageInfoField::CurrentPage => "currentPage".to_owned(),
            PageInfoField::LastPage => "lastPage".to_owned(),
            PageInfoField::HasNextPage => "hasNextPage".to_owned(),
        }
    }
}
