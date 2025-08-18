# rust-anki-connect-wrapper

A thin wrapper around the [Anki-Connect REST API](https://git.sr.ht/~foosoft/anki-connect) for [Anki](https://apps.ankiweb.net/), written in Rust.

## Usage

Create a client wrapper

```rust
let anki = ankiconnect::AnkiConnect::builder()
    .with_url(args.url)
    .build();
```

Make an API request

```rust
let query = "prop:ivl>=21";

let notes = match anki.find_notes(query).await {
    Ok(x) => x,
    Err(e) => {
        eprintln!("Failed to search for notes: {e}");
        std::process::exit(1);
    },
};

eprintln!("Found {} notes", notes.len());
```

## Help wanted

Only a small subset of all of the API endpoints have been implemented. Pull requests to add support for more endpoints are welcome.

