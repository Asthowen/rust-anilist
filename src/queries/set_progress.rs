pub const SET_PROGRESS: &str = r#"
mutation ($mediaId: Int, $progress: Int, $hidden: Boolean) {
    SaveMediaListEntry(mediaId: $mediaId, progress: $progress, hiddenFromStatusLists: $hidden) {
        progress
        hiddenFromStatusLists
    }
}
"#;
