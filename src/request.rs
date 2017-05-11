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

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json;
use super::builder::Search;
use super::model::{Anime, Manga, Response, User};
use super::Result;

static API_URL: &'static str = "https://kitsu.io/api/edge";

/// Retrieves data about an anime by its Id.
pub fn get_anime(id: u64) -> Result<Response<Anime>> {
    let ssl = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(ssl);
    let response = Client::with_connector(connector)
        .get(&format!("{}/anime/{}", API_URL, id))
        .send()?;

    let x: Response<Anime> = serde_json::from_reader(response)?;

    Ok(x)
}

/// Retrieves data about a user by its id.
pub fn get_user(id: u64) -> Result<Response<User>> {
    let ssl = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(ssl);
    let response = Client::with_connector(connector)
        .get(&format!("{}/users/{}", API_URL, id))
        .send()?;

    let x: Response<User> = serde_json::from_reader(response)?;

    Ok(x)
}

/// Retrieves data about a manga by its id.
pub fn get_manga(id: u64) -> Result<Response<Manga>> {
    let ssl = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(ssl);
    let response = Client::with_connector(connector)
        .get(&format!("{}/manga/{}", API_URL, id))
        .send()?;

    let x: Response<Manga> = serde_json::from_reader(response)?;

    Ok(x)
}

/// Searches anime with optional parameters via a builder.
pub fn search_anime<F: FnOnce(Search) -> Search>(f: F) -> Result<Response<Vec<Anime>>> {
    let params = f(Search::default()).0;

    let ssl = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(ssl);
    let response = Client::with_connector(connector)
        .get(&format!("{}/anime?{}", API_URL, params))
        .send()?;

    let x: Response<Vec<Anime>> = serde_json::from_reader(response)?;

    Ok(x)
}

/// Searches manga with optional parameters via a builder.
pub fn search_manga<F: FnOnce(Search) -> Search>(f: F)-> Result<Response<Vec<Manga>>> {
    let params = f(Search::default()).0;

    let ssl = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(ssl);
    let response = Client::with_connector(connector)
        .get(&format!("{}/manga?{}", API_URL, params))
        .send()?;

    use hyper::client::Response as HyperResponse;

    Ok(serde_json::from_reader::<HyperResponse, Response<Vec<Manga>>>(response)?)
}

/// Searches users with optional parameters via a builder.
pub fn search_users<F: FnOnce(Search) -> Search>(f: F) -> Result<Response<Vec<User>>> {
    let params = f(Search::default()).0;

    let ssl = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(ssl);
    let response = Client::with_connector(connector)
        .get(&format!("{}/users?{}", API_URL, params))
        .send()?;

    let x: Response<Vec<User>> = serde_json::from_reader(response)?;

    Ok(x)
}

#[cfg(test)]
mod tests {
    #[test]
    fn anime() {
        let _ = ::search_anime(|f| f.filter("text", "non non biyori")).expect("nnb");
        let _ = ::get_anime(7711).expect("1 res ret");

        // Test that AgeRating TV-Y7 is handled. Undocumented age rating.
        let _ = ::search_anime(|f| f.filter("text", "Avatar")).expect("avatar");
    }
    #[test]
    fn users() {
        let _ = ::search_users(|f| f.filter("query", "Josh")).expect("Josh");
        let _ = ::get_user(1).expect("vikhyat");
    }
}
