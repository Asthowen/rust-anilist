use crate::builders::BuiltQuery;
use crate::builders::fields::MediaField;
use crate::models::MediaType;
use serde_json::{Map, Value};

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
            id: None,
            media_type: None,
            search_text: None,
            html_description: true,
            fields: MediaField::all().to_vec(),
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
        let index_string = index.to_string();

        let parts: Vec<String> = self
            .fields
            .iter()
            .map(|&field| {
                let field_string: String = field.into();
                field_string.replace("%%index%%", &index_string)
            })
            .collect::<Vec<String>>();

        let mut variables = Map::default();
        let mut parameters = Vec::new();
        let mut parameters_definitions = Vec::new();

        if let Some(id) = self.id {
            let var_name = format!("id{index}");
            parameters.push(format!("id: ${var_name}"));
            parameters_definitions.push(format!("${var_name}: Int"));
            variables.insert(var_name, Value::from(id));
        }

        if let Some(media_type) = self.media_type {
            parameters.push(format!("type: {media_type}"));
        }

        if let Some(search_text) = &self.search_text {
            let var_name = format!("search{index}");
            parameters.push(format!("search: ${var_name}"));
            parameters_definitions.push(format!("${var_name}: String!"));
            variables.insert(var_name, Value::from(search_text.as_str()));
        }

        {
            let var_name = format!("htmlDescription{index}");
            parameters_definitions.push(format!("${var_name}: Boolean"));
            variables.insert(var_name, Value::from(self.html_description));
        }

        BuiltQuery {
            field_fragment: format!(
                "Media ({}) {{ {} }}",
                parameters.join(", "),
                parts.join(" ")
            ),
            variables,
            parameters_definitions,
        }
    }
}
