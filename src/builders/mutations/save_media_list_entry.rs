use crate::builders::fields::{JoinFields, MediaField, MediaListField};
use crate::builders::{BuiltQuery, ParametersBuilder};
use crate::models::{FuzzyDate, MediaListStatus};
use serde_json::Value;

#[derive(Default)]
pub struct SaveMediaListEntryMutationBuilder<'a> {
    id: Option<i32>,
    media_id: Option<i32>,
    status: Option<MediaListStatus>,
    score: Option<f64>,
    score_raw: Option<i32>,
    progress: Option<i32>,
    volumes_progress: Option<i32>,
    repeat: Option<i32>,
    priority: Option<i32>,
    private: Option<bool>,
    notes: Option<String>,
    hidden_from_status_lists: Option<bool>,
    custom_lists: Option<Vec<String>>,
    advanced_scores: Option<Vec<f64>>,
    started_at: Option<FuzzyDate>,
    completed_at: Option<FuzzyDate>,
    html_description: bool,
    fields: Vec<MediaListField<'a>>,
}
impl<'a> SaveMediaListEntryMutationBuilder<'a> {
    pub fn new() -> Self {
        Self {
            html_description: true,
            ..Default::default()
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);

        self
    }

    pub fn with_media_id(mut self, media_id: i32) -> Self {
        self.media_id = Some(media_id);

        self
    }

    pub fn set_status_to(mut self, status: MediaListStatus) -> Self {
        self.status = Some(status);

        self
    }

    pub fn set_score_to(mut self, score: f64) -> Self {
        self.score = Some(score);

        self
    }

    pub fn set_score_raw_to(mut self, score_raw: i32) -> Self {
        self.score_raw = Some(score_raw);

        self
    }

    pub fn set_progress_to(mut self, progress: i32) -> Self {
        self.progress = Some(progress);

        self
    }

    pub fn set_volumes_progress_to(mut self, volumes_progress: i32) -> Self {
        self.volumes_progress = Some(volumes_progress);

        self
    }

    pub fn set_repeat_to(mut self, repeat: i32) -> Self {
        self.repeat = Some(repeat);

        self
    }

    pub fn set_private_to(mut self, private: bool) -> Self {
        self.private = Some(private);

        self
    }

    pub fn set_notes_to(mut self, notes: String) -> Self {
        self.notes = Some(notes);

        self
    }

    pub fn set_hidden_from_status_lists_to(mut self, hidden_from_status_lists: bool) -> Self {
        self.hidden_from_status_lists = Some(hidden_from_status_lists);

        self
    }

    pub fn add_to_custom_lists(mut self, custom_lists: Vec<String>) -> Self {
        self.custom_lists = Some(custom_lists);

        self
    }

    pub fn set_advanced_scores_to(mut self, advanced_scores: Vec<f64>) -> Self {
        self.advanced_scores = Some(advanced_scores);

        self
    }

    pub fn set_started_at_to(mut self, started_at: FuzzyDate) -> Self {
        self.started_at = Some(started_at);

        self
    }

    pub fn set_completed_at_to(mut self, completed_at: FuzzyDate) -> Self {
        self.completed_at = Some(completed_at);

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

    pub fn with_fields(mut self, fields: &[MediaListField<'a>]) -> Self {
        self.fields.extend_from_slice(fields);

        self
    }

    pub fn with_field(mut self, field: MediaListField<'a>) -> Self {
        self.fields.push(field);

        self
    }

    pub(crate) fn build(&self, index: usize) -> BuiltQuery {
        let mut fields = self.fields.clone();
        if self.fields.is_empty() {
            fields.push(MediaListField::MediaId);
        }

        let parts: Vec<String> = fields.join_fields_index(index);

        let mut parameters = ParametersBuilder::new(index);
        parameters.insert("id", "Int", self.id.map(Value::from));
        parameters.insert("mediaId", "Int", self.media_id.map(Value::from));
        parameters.insert(
            "status",
            "MediaListStatus",
            self.status
                .and_then(|value| serde_json::to_value(value).ok()),
        );
        parameters.insert("score", "Float", self.score.map(Value::from));
        parameters.insert("scoreRaw", "Int", self.score_raw.map(Value::from));
        parameters.insert("progress", "Int", self.progress.map(Value::from));
        parameters.insert(
            "progressVolumes",
            "Int",
            self.volumes_progress.map(Value::from),
        );
        parameters.insert("repeat", "Int", self.repeat.map(Value::from));
        parameters.insert("priority", "Int", self.priority.map(Value::from));
        parameters.insert("private", "Boolean", self.private.map(Value::from));
        parameters.insert("notes", "String", self.notes.clone().map(Value::from));
        parameters.insert(
            "hiddenFromStatusLists",
            "Boolean",
            self.hidden_from_status_lists.map(Value::from),
        );
        parameters.insert(
            "customLists",
            "[String]",
            self.custom_lists.clone().map(Value::from),
        );
        parameters.insert(
            "advancedScores",
            "[Float]",
            self.advanced_scores.clone().map(Value::from),
        );
        parameters.insert(
            "startedAt",
            "FuzzyDateInput",
            self.started_at.map(Value::from),
        );
        parameters.insert(
            "completedAt",
            "FuzzyDateInput",
            self.completed_at.map(Value::from),
        );
        if fields.iter().any(|field|
            matches!(field, MediaListField::Media(media_fields) if media_fields.contains(&MediaField::Description))
        ) {
            parameters.insert_local_variable(
                "htmlDescription",
                "Boolean",
                Some(Value::from(self.html_description)),
            );
        }

        parameters.build("SaveMediaListEntry", parts)
    }
}
