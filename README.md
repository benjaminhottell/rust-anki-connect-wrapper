# rust-anki-connect-wrapper

A thin wrapper around the [Anki-Connect REST API](https://git.sr.ht/~foosoft/anki-connect) for [Anki](https://apps.ankiweb.net/), written in Rust.

## Usage

Create a client wrapper

```rust
let anki = ankiconnect::Client::builder()
    .with_url(args.url)
    .build();
```

Make an API request

```rust
let browse_query = "prop:ivl>=21";

let browse_request = GuiBrowse::builder()
    .query(browse_query)
    .build();

if let Err(e) = anki.invoke(&browse_request).await {
    eprintln!("Failed to display browser: {e}");
    std::process::exit(1);
};
```

## Help wanted

Only a small subset of all of the API endpoints have been implemented. Pull requests to add support for more endpoints are welcome.

