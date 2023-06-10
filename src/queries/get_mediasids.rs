pub const GET_MEDIAS_IDS: &str = r#"
query MediaListEntry($id: Int) {
  Media(id: $id) {
    mediaListEntry {
      id
    }
  }
}
"#;
