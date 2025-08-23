#[derive(Copy, Clone)]
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

impl From<PageInfoField> for &'static str {
    fn from(value: PageInfoField) -> Self {
        match value {
            PageInfoField::Total => "total",
            PageInfoField::PerPage => "perPage",
            PageInfoField::CurrentPage => "currentPage",
            PageInfoField::LastPage => "lastPage",
            PageInfoField::HasNextPage => "hasNextPage",
        }
    }
}
