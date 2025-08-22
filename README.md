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

## Testing

### Unit testing

Unit tests can be executed without an Anki instance running. These test internal components only, and do not make any connections to Anki or Anki-Connect.

```rust
cargo test --lib
```

You can also run the doctests without an Anki instance running.

```rust
cargo test --doc
```

### Integration testing

Integration tests require an Anki instance with the Anki-Connect plugin installed, available at the default port on localhost.

The `stateless` tests will run a series of non-modifying API requests (e.g. `version`). These should not modify the state of the Anki instance, and should therefore be 'safe' to run.

```rust
cargo test --tests stateless
```

In the future modifying tests could be added so it is best to specify `stateless` explicitly.

## Help wanted

Only a small subset of all of the API endpoints have been implemented. Pull requests to add support for more endpoints are welcome.

