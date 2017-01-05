// ISC License (ISC)
//
// Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
//! An unofficial Rust library acting as a wrapper around the [Kitsu] API.
//!
//! The library supports retrieval from the API, but does not currently support
//! authenticated requests.
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependences]
//! kitsu_io = "0.1"
//! ```
//!
//! and at the top of your `main.rs` or `lib.rs`:
//!
//! ```rust
//! extern crate kitsu_io;
//! ```
//!
//! # Example
//!
//! Retrieve anime matching the content `"non non biyori"`:
//!
//! ```rust
//! use kitsu_io;
//!
//! let _media = kitsu_io::search_anime(|f| f.filter("text", "non non biyori")).unwrap();
//! ```
//!
//! # License
//!
//! ISC. View the full license [here][license file].
//!
//! [Kitsu]: https://kitsu.io
//! [license file]: https://raw.githubusercontent.com/zeyla/kitsu.rs/master/LICENSE.md
#![warn(missing_docs)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

pub mod builder;
pub mod model;

mod error;
mod request;

pub use error::{Error, Result};
pub use request::*;
