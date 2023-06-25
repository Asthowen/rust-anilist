pub const SET_PROGRESS_INCREMENT: &str = r#"
mutation ($mediaId: Int, $progress_start: Int, $progress_end: Int, $hidden: Boolean) {
  saveMediaListEntry1: SaveMediaListEntry(mediaId: $mediaId, progress: $progress_start, hiddenFromStatusLists: $hidden) {
    progress
    hiddenFromStatusLists
  }
  saveMediaListEntry2: SaveMediaListEntry(mediaId: $mediaId, progress: $progress_end, hiddenFromStatusLists: $hidden) {
    progress
    hiddenFromStatusLists
  }
}
"#;

pub const SET_PROGRESS: &str = r#"
mutation ($mediaId: Int, $progress: Int, $hidden: Boolean) {
    SaveMediaListEntry(mediaId: $mediaId, progress: $progress, hiddenFromStatusLists: $hidden) {
        progress
        hiddenFromStatusLists
    }
}
"#;
