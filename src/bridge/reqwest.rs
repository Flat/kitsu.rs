//! Bridge to provide a client implementation for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`KitsuRequester`].
//!
//! [`KitsuRequester`]: trait.KitsuRequester.html

use ::builder::Search;
use ::model::{Anime, Manga, Response, User};
use reqwest::blocking::{Client as ReqwestClient, RequestBuilder};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json;
use std::io::Read;
use ::{Error, Result, API_URL};

/// Trait which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the implemenation for the `reqwest` Client, simply use the
/// trait:
///
/// ```rust,no_run
/// use kitsu_io::KitsuReqwestRequester;
/// ```
///
/// At this point, the methods will be on your Reqwest Client.
pub trait KitsuRequester {
    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_io;
    /// extern crate reqwest;
    ///
    /// use kitsu_io::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let anime_id = 1;
    ///
    ///     // Get the anime.
    ///     let anime = client.get_anime(anime_id)
    ///         .expect("Error getting anime");
    ///
    ///     // Do something with anime
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn get_anime(&self, id: u64) -> Result<Response<Anime>>;

    /// Gets a manga using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_io;
    /// extern crate reqwest;
    ///
    /// use kitsu_io::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let manga_id = 1;
    ///
    ///     // Get the manga.
    ///     let manga = client.get_anime(manga_id)
    ///         .expect("Error getting manga");
    ///
    ///     // Do something with manga
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn get_manga(&self, id: u64) -> Result<Response<Manga>>;

    /// Gets a user using their id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_io;
    /// extern crate reqwest;
    ///
    /// use kitsu_io::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let user_id = 1;
    ///
    ///     // Get the user.
    ///     let user = client.get_anime(user_id)
    ///         .expect("Error getting user");
    ///
    ///     // Do something with user
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn get_user(&self, id: u64) -> Result<Response<User>>;

    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_io;
    /// extern crate reqwest;
    ///
    /// use kitsu_io::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let anime_name = "Your Lie in April";
    ///
    ///     // Search for the anime.
    ///     let anime = client.search_anime(|f| f.filter("text", anime_name))
    ///         .expect("Error searching for anime");
    ///
    ///     // Do something with anime
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn search_anime<F: FnOnce(Search) -> Search>(&self, f: F) -> Result<Response<Vec<Anime>>>;

    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_io;
    /// extern crate reqwest;
    ///
    /// use kitsu_io::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let manga_name = "Say I Love You";
    ///
    ///     // Search for the manga.
    ///     let manga = client.search_manga(|f| f.filter("text", manga_name))
    ///         .expect("Error getting manga");
    ///
    ///     // Do something with manga
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn search_manga<F: FnOnce(Search) -> Search>(&self, f: F) -> Result<Response<Vec<Manga>>>;

    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_io;
    /// extern crate reqwest;
    ///
    /// use kitsu_io::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let user_name = "Billy";
    ///
    ///     // Search for the user.
    ///     let user = client.search_users(|f| f.filter("name", user_name))
    ///         .expect("Error searching for user");
    ///
    ///     // Do something with users
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn search_users<F: FnOnce(Search) -> Search>(&self, f: F) -> Result<Response<Vec<User>>>;
}

impl KitsuRequester for ReqwestClient {
    fn get_anime(&self, id: u64) -> Result<Response<Anime>> {
        let uri = url::Url::parse(&format!("{}/anime/{}", API_URL, id.to_string()))?;

        handle_request::<Response<Anime>>(self.get(uri))
    }

    fn get_manga(&self, id: u64) -> Result<Response<Manga>> {
        let uri = url::Url::parse(&format!("{}/manga/{}", API_URL, id.to_string()))?;

        handle_request::<Response<Manga>>(self.get(uri))
    }

    fn get_user(&self, id: u64) -> Result<Response<User>> {
        let uri = url::Url::parse(&format!("{}/users/{}", API_URL, id.to_string()))?;

        handle_request::<Response<User>>(self.get(uri))
    }

    fn search_anime<F: FnOnce(Search) -> Search>(&self, f: F) -> Result<Response<Vec<Anime>>> {
        let params = f(Search::default()).0;
        let uri = url::Url::parse(&format!("{}/anime?{}", API_URL, params))?;

        handle_request::<Response<Vec<Anime>>>(self.get(uri))
    }

    fn search_manga<F: FnOnce(Search) -> Search>(&self, f: F) -> Result<Response<Vec<Manga>>> {
        let params = f(Search::default()).0;
        let uri = url::Url::parse(&format!("{}/manga?{}", API_URL, params))?;

        handle_request::<Response<Vec<Manga>>>(self.get(uri))
    }

    fn search_users<F: FnOnce(Search) -> Search>(&self, f: F) -> Result<Response<Vec<User>>> {
        let params = f(Search::default()).0;
        let uri = url::Url::parse(&format!("{}/users?{}", API_URL, params))?;

        handle_request::<Response<Vec<User>>>(self.get(uri))
    }
}

fn handle_request<T: DeserializeOwned>(request: RequestBuilder) -> Result<T> {
    let response = request.send()?;

    match response.status() {
        StatusCode::OK => {}
        StatusCode::BAD_REQUEST => {
            return Err(Error::ReqwestBad());
        }
        StatusCode::UNAUTHORIZED => {
            return Err(Error::ReqwestUnauthorized());
        }
        _ => return Err(Error::ReqwestInvalid()),
    }

    from_reader(response)
}

#[inline]
fn from_reader<T: DeserializeOwned, U: Read>(reader: U) -> Result<T> {
    serde_json::from_reader(reader).map_err(From::from)
}
