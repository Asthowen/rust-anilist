use crate::builders::fields::{JoinFields, MediaField};
use crate::builders::{BuiltQuery, ParametersBuilder};
use crate::models::MediaType;
use serde_json::Value;

#[derive(Default)]
pub struct MediaQueryBuilder<'a> {
    id: Option<i32>,
    media_type: Option<MediaType>,
    search_text: Option<String>,
    html_description: bool,
    fields: Vec<MediaField<'a>>,
}
impl<'a> MediaQueryBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_all_fields() -> Self {
        Self {
            html_description: true,
            fields: MediaField::all().to_vec(),
            ..Default::default()
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);

        self
    }

    pub fn with_media_type(mut self, media_type: MediaType) -> Self {
        self.media_type = Some(media_type);

        self
    }

    pub fn with_text_search<S: Into<String>>(mut self, text: S) -> Self {
        self.search_text = Some(text.into());

        self
    }

    pub fn with_markdown_description(mut self) -> Self {
        self.html_description = false;

        self
    }

    pub fn with_html_description(mut self) -> Self {
        self.html_description = true;

        self
    }

    pub fn with_fields(mut self, fields: &[MediaField<'a>]) -> Self {
        self.fields.extend_from_slice(fields);

        self
    }

    pub fn with_field(mut self, field: MediaField<'a>) -> Self {
        self.fields.push(field);

        self
    }

    pub(crate) fn build(&self, index: usize) -> BuiltQuery {
        let mut fields = self.fields.clone();
        if self.fields.is_empty() {
            fields.push(MediaField::Id);
        }

        let parts: Vec<String> = fields.join_fields_index(index);

        let mut parameters = ParametersBuilder::new(index);
        parameters.insert("id", "Int", self.id.map(Value::from));
        parameters.insert_local_parameter(
            "type",
            self.media_type
                .and_then(|value| serde_json::to_value(value).ok()),
        );
        parameters.insert(
            "search",
            "String!",
            self.search_text.as_deref().map(Value::from),
        );
        if fields.contains(&MediaField::Description) {
            parameters.insert_local_variable(
                "htmlDescription",
                "Boolean",
                Some(Value::from(self.html_description)),
            );
        }

        parameters.build("Media", parts)
    }
}
