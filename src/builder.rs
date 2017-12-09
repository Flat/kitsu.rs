//! A set of builders for ease of use with optional parameters around the API.

use std::fmt::Write;

/// Filters search results.
///
/// The following are filters in addition to each search type's fields:
///
/// - `search_anime`: `season`, `streamers`, `text`
/// - `search_manga]: `text`
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
