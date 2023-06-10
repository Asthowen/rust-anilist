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
### Create client
```rust
let anilist_client = AniListClientBuilder::builder()
    .with_initialized_reqwest_client()
    .with_anilist_token("some")
    .build()?;
```

### Set manga progress
```rust
anilist_client.set_progress(139741, 67, "access token").await;
```

## License
**[rust-anilist](https://github.com/Asthowen/rust-anilist) | Copyright ©Asthowen 2022 - now | All rights reserved**