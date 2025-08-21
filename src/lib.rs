// Relevant docs: https://git.sr.ht/~foosoft/anki-connect

pub mod error;
pub mod client;
pub mod models;
pub mod endpoints;

pub use client::Client;
pub use error::Error;
