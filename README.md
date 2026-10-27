<!--suppress HtmlDeprecatedAttribute -->
<div align="center">
    <h1>rust-anilist</h1>
    <p>
    <a href="https://www.rust-lang.org/">
        <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Made with Rust">
    </a>
        <a href="https://github.com/Asthowen/rust-anilist">
            <img src="https://img.shields.io/badge/Git-F05032?style=for-the-badge&logo=git&logoColor=white" alt="Use git">
        </a>
    </p>
    <h3>
        <strong>rust-anilist is a Rust client wrapper for AniList API.</strong>
    </h3>
</div>

## Examples
### Add to deps
```toml
[dependencies]
anilist = { git = "https://github.com/Asthowen/rust-anilist", version = "0.1.1" }
```

### Create client
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let anilist_client = AniListClient::builder()
        .build()?;

    Ok(())
}
```

### Retrieving manga from ID `64127`
```rust
anilist_client
    .send_query(MediaQueryBuilder::new_all_fields().with_id(64127), None)
    .await?;
```

### Retrieving some of the manga information from ID `64127`
```rust
anilist_client
    .send_query(
        MediaQueryBuilder::new()
            .with_fields(&[
                MediaField::Id,
                MediaField::Title(MediaTitleField::all()),
                MediaField::Tags(&[MediaTagField::Id, MediaTagField::Name]),
            ])
            .with_id(64127),
        None,
    )
    .await?;
```

### Retrieving manga from IDs `64127` and `139741` in a single request
```rust
anilist_client
    .send_queries(
        &[
            MediaQueryBuilder::new_all_fields().with_id(64127),
            MediaQueryBuilder::new_all_fields().with_id(139741),
        ],
        None,
    )
    .await?;
```

### Marking chapter 5 of manga `64127` as read
```rust
anilist_client
    .send_query(
        SaveMediaListEntryMutationBuilder::new()
            .with_fields(&[MediaListField::MediaId])
            .with_media_id(64127)
            .set_progress_to(5),
        Some("token"),
    )
    .await?;
```

### Marking volume 5 of manga `64127` as read
```rust
anilist_client
    .send_query(
        SaveMediaListEntryMutationBuilder::new()
            .with_fields(&[MediaListField::MediaId])
            .with_media_id(64127)
            .set_volumes_progress_to(5),
        Some("token"),
    )
    .await?;
```

### Marking chapters 5-15 of manga `64127` as read
```rust
anilist_client
    .send_queries(
        &[
            SaveMediaListEntryMutationBuilder::new()
                .with_fields(&[MediaListField::MediaId])
                .with_media_id(64127)
                .set_progress_to(5),
            SaveMediaListEntryMutationBuilder::new()
                .with_fields(&[MediaListField::MediaId])
                .with_media_id(64127)
                .set_progress_to(15),
        ],
        Some("token"),
    )
    .await?;
```

## Contributions
**Before committing an update:**
* The code must have no warning with clippy, use the command: `cargo clippy --all-features --all-targets -- -D warnings`
* The code must be cleaned with cargo fmt, use the command: `cargo fmt`

## License
**[rust-anilist](https://github.com/Asthowen/rust-anilist) | [GNU General Public License v3.0](https://github.com/Asthowen/rust-anilist/blob/main/LICENSE)**
