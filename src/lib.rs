//! # kitsu.rs
//!
//! An unofficial Rust library acting as a wrapper around the [Kitsu] API, offering
//! implementations for both asynchronous hyper(v0.11) and synchronous reqwest(0.8.0).
//!
//! **note:** The library supports retrieval from the API, but does not currently
//! support authenticated requests.
//!
//! ### Compile features
//!
//! - **hyper-support**: Compiles with `hyper` support
//! - **reqwest-support**: Compliles with `reqwest` support (*default*)
//!
//! ### Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! kitsu_io = "0.2"
//! ```
//!
//! To enable both `hyper` and `reqwest` support:
//!
//! ```toml
//! [dependencies.kitsu_io]
//! version = "0.2"
//! features = ["hyper-support", "reqwest-support"]
//! ```
//!
//! To enable `hyper` but not `reqwest` support:
//!
//! ```toml
//! [dependencies.kitsu_io]
//! version = "0.2"
//! default-features = false
//! features = ["hyper-support"]
//! ```
//!
//! ### Examples
//!
//! Using reqwest, search for an anime using a string taken from user input:
//!
//! ```rust,no_run
//! extern crate kitsu_io;
//! # #[cfg(feature = "reqwest")]
//! extern crate reqwest;
//!
//!
//! # #[cfg(feature = "reqwest")]
//! # fn main() {
//! #
//! use kitsu_io::KitsuReqwestRequester;
//! use reqwest::Client;
//! use std::io::{self, Write};
//!
//! // Create the reqwest Client.
//! let client = Client::new();
//!
//! // Read an anime name to search for from the users input.
//! let mut input = String::new();
//! print!("Enter an anime name to search for:\n>");
//! let _ = io::stdout().flush();
//! io::stdin().read_line(&mut input).expect("Error reading input");
//! let input_trimmed = input.trim();
//!
//! // Search for the anime.
//! let anime = client.search_anime(|f| f.filter("text", input_trimmed))
//!     .expect("Error searching for anime");
//!
//! // Print out the response of the request.
//! if let Some(ref picked) = anime.data.first() {
//!     let title = &picked.attributes.canonical_title;
//!
//!     if let Some(ref rating) = picked.attributes.average_rating {
//!         println!("Found Anime: {} - {}", title, rating);
//!     } else {
//!        println!("Found Anime: {} - ??", title);
//!     }
//! } else {
//!     println!("No Anime Found.");
//! }
//! # }
//! # #[cfg(not(feature = "reqwest"))]
//! # fn main() { }
//! ```
//!
//! For more examples, refer to the [examples] folder.
//!
//! ### License
//!
//! ISC. View the full license [here][license file].
//!
//! [Kitsu]: https://kitsu.io
//! [examples]: https://github.com/zeyla/kitsu.rs/blob/master/examples
//! [license file]: https://github.com/zeyla/kitsu.rs/blob/master/README.md
#![deny(missing_docs)]

#[cfg(feature = "hyper")]
extern crate hyper;
#[cfg(feature = "hyper")]
extern crate hyper_tls;
#[cfg(feature = "reqwest")]
extern crate reqwest;
#[cfg(feature = "serde_derive")]
extern crate serde;
#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "serde_derive")]
extern crate serde_json;

pub mod bridge;
pub mod builder;

#[cfg(feature = "serde_derive")]
pub mod model;

mod error;

pub use error::{Error, Result};

#[cfg(feature = "hyper")]
pub use bridge::hyper::KitsuRequester as KitsuHyperRequester;
#[cfg(feature = "reqwest")]
pub use bridge::reqwest::KitsuRequester as KitsuReqwestRequester;

/// Kitsu API Url
pub const API_URL: &'static str = "https://kitsu.io/api/edge";
