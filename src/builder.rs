// ISC License (ISC)
//
// Copyright (c) 2016, Zeyla Hellyer <zey@zey.moe>
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
//! A set of builders for ease of use with optional parameters around the API.

use std::fmt::Write;

/// Filters search results.
///
/// The following are filters in addition to each search type's fields:
///
/// - [`search_anime`]: `season`, `streamers`, `text`
/// - [`search_manga`]: `text`
///
/// [`search_anime`]: fn.search_anime.html
/// [`search_manga`]: fn.search_manga.html
#[derive(Default)]
pub struct Search(pub String);

impl Search {
    /// Filters results by a key and value.
    pub fn filter(mut self, key: &str, value: &str) -> Self {
        let _ = write!(self.0, "&filter[{}]={}", key, value);

        self
    }

    /// Sets a limit to the number of results that can be returned.
    ///
    /// This is used for pagination, in conjunction with [`offset`].
    ///
    /// [`offset`]: #method.offset
    pub fn limit(mut self, limit: u64) -> Self {
        let _ = write!(self.0, "&page[limit]={}", limit);

        self
    }

    /// Sets an offset to the number of results that can be returned.
    ///
    /// This is used for pagination, in conjunction with [`limit`].
    ///
    /// [`limit`]: #method.limit
    pub fn offset(mut self, offset: u64) -> Self {
        let _ = write!(self.0, "&page[offset]={}", offset);

        self
    }

    /// Sets a sorting order to use by specifying fields.
    ///
    /// `id` will sort ascending, while `-id` will sort descending. Multiple
    /// sorters can be provided by joining with a comma (`','`).
    pub fn sort(mut self, sort: &str) -> Self {
        let _ = write!(self.0, "&sort={}", sort);

        self
    }
}
