//! Bridge to provide a client implementation for the `hyper` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`KitsuRequester`].
//!
//! [`KitsuRequester`]: trait.KitsuRequester.html

use hyper::client::{Client as HyperClient, FutureResponse, HttpConnector};
use hyper::{Body, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use std::str::FromStr;
use ::builder::Search;
use ::{API_URL, Result};

/// Trait which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the implemenation for the `hyper` Client, simply use the
/// trait:
///
/// ```rust,no_run
/// use kitsu_io::KitsuHyperRequester;
/// ```
///
/// At this point, the methods will be on your Hyper Client.
pub trait KitsuRequester {
    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    /// Get an anime with the id of 1:
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let anime_id = 1;
    ///
    /// let runner = client.get_anime(anime_id)?
    ///     .and_then(|res| {
    ///         res.body().for_each(|chunk| {
    ///             io::stdout().write_all(&chunk).map_err(From::from)
    ///         })
    ///     }).map(|_| {
    ///         println!("\n\nDone.");
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn get_anime(&self, id: u64) -> Result<FutureResponse>;

    /// Gets a manga using its id.
    ///
    /// # Examples
    ///
    /// Get a manga with the id of 1:
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let manga_id = 1;
    ///
    /// let runner = client.get_manga(manga_id)?
    ///     .and_then(|res| {
    ///         res.body().for_each(|chunk| {
    ///             io::stdout().write_all(&chunk).map_err(From::from)
    ///         })
    ///     }).map(|_| {
    ///         println!("\n\nDone.");
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn get_manga(&self, id: u64) -> Result<FutureResponse>;

    /// Gets a user using their id.
    ///
    /// # Examples
    ///
    /// Get a user with the id of 1:
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let user_id = 1;
    ///
    /// let runner = client.get_user(user_id)?
    ///     .and_then(|res| {
    ///         res.body().for_each(|chunk| {
    ///             io::stdout().write_all(&chunk).map_err(From::from)
    ///         })
    ///     }).map(|_| {
    ///         println!("\n\nDone.");
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn get_user(&self, id: u64) -> Result<FutureResponse>;

    /// Searches for an anime using the passed [Search] builder.
    ///
    /// # Examples
    ///
    /// Search for an anime with the name "Beyond the Boundary":
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let anime_name = "Beyond the Boundary";
    ///
    /// let runner = client.search_anime(|f| f.filter("text", anime_name))?
    ///     .and_then(|res| {
    ///         res.body().for_each(|chunk| {
    ///             io::stdout().write_all(&chunk).map_err(From::from)
    ///         })
    ///     }).map(|_| {
    ///         println!("\n\nDone.");
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn search_anime<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<FutureResponse>;

    /// Searches for a manga using the passed [Search] builder.
    ///
    /// # Examples
    ///
    /// Search for a manga with the name "Orange":
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let manga_name = "Orange";
    ///
    /// let runner = client.search_manga(|f| f.filter("text", manga_name))?
    ///     .and_then(|res| {
    ///         res.body().for_each(|chunk| {
    ///             io::stdout().write_all(&chunk).map_err(From::from)
    ///         })
    ///     }).map(|_| {
    ///         println!("\n\nDone.");
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn search_manga<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<FutureResponse>;

    /// Searches for a user using the passed [`Search`] builder.
    ///
    /// # Examples
    ///
    /// Search for a user with the name "Bob":
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let user_name = "Bob";
    ///
    /// let runner = client.search_users(|f| f.filter("name", user_name))?
    ///     .and_then(|res| {
    ///         res.body().for_each(|chunk| {
    ///             io::stdout().write_all(&chunk).map_err(From::from)
    ///         })
    ///     }).map(|_| {
    ///         println!("\n\nDone.");
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    /// [`Search`]: ../builder/struct.Search.html
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn search_users<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<FutureResponse>;
}

impl KitsuRequester for HyperClient<HttpsConnector<HttpConnector>, Body> {
    fn get_anime(&self, id: u64) -> Result<FutureResponse> {
        let uri = Uri::from_str(&format!("{}/anime/{}", API_URL, id))?;
        let request = Request::new(Method::Get, uri);

        Ok(self.request(request))
    }

    fn get_manga(&self, id: u64) -> Result<FutureResponse> {
        let uri = Uri::from_str(&format!("{}/manga/{}", API_URL, id))?;
        let request = Request::new(Method::Get, uri);

        Ok(self.request(request))
    }

    fn get_user(&self, id: u64) -> Result<FutureResponse> {
        let uri = Uri::from_str(&format!("{}/users/{}", API_URL, id))?;
        let request = Request::new(Method::Get, uri);

        Ok(self.request(request))
    }

    fn search_anime<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<FutureResponse> {
        let params = f(Search::default()).0;

        let uri = Uri::from_str(&format!("{}/anime?{}", API_URL, params))?;
        let request = Request::new(Method::Get, uri);

        Ok(self.request(request))
    }

    fn search_manga<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<FutureResponse> {
        let params = f(Search::default()).0;

        let uri = Uri::from_str(&format!("{}/manga?{}", API_URL, params))?;
        let request = Request::new(Method::Get, uri);

        Ok(self.request(request))
    }

    fn search_users<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<FutureResponse> {
        let params = f(Search::default()).0;

        let uri = Uri::from_str(&format!("{}/users?{}", API_URL, params))?;
        let request = Request::new(Method::Get, uri);

        Ok(self.request(request))
    }
}
