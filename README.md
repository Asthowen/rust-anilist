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
        <strong>rust-anilist is a Rust client wrapper for AniList website.</strong>
    </h3>
</div>

## Dev
**Before committing an update:**
* The code must have no warning with clippy, use the command: `clippy`
* The code must be cleaned with cargo fmt, run the command: `cargo fmt`

## Examples
### Add to deps
```toml
[dependencies]
anilist = { git = "https://github.com/Asthowen/rust-anilist", version = "0.0.2" }
```

### Create client
```rust
let anilist_client = AniListClientBuilder::builder()
    .with_initialized_reqwest_client()
    .with_anilist_token("some")
    .build()?;
```

### Set manga progress
```rust
anilist_client.set_progress(67, 139741, "access token").await;
```

### Set manga increment progress
```rust
anilist_client.set_increment_progress(1, 67, 139741, "access token").await;
```

## Thanks
Thanks to [AndrielFR](https://github.com/AndrielFR) for the models: https://github.com/AndrielFR/rust-anilist

## License
**[rust-anilist](https://github.com/Asthowen/rust-anilist) | [GNU General Public License v3.0](https://github.com/Asthowen/rust-anilist/blob/main/LICENSE)**