An unofficial Rust library acting as a wrapper around the [Kitsu] API.

The library supports retrieval from the API, but does not currently support
authenticated requests.

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependences]
kitsu_io = "0.1"
```

and at the top of your `main.rs` or `lib.rs`:

```rust
extern crate kitsu_io;
```

# Example

Retrieve anime matching the content `"non non biyori"`:

```rust
use kitsu_io;

let _media = kitsu_io::search_anime(|f| f.filter("text", "non non biyori")).unwrap();
```

# License

ISC. View the full license [here][license file].

[Kitsu]: https://kitsu.io
[license file]: https://github.com/zeyla/kitsu.rs/blob/master/README.md
