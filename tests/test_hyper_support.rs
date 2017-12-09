#![cfg(feature = "hyper-support")]

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate kitsu_io;
extern crate tokio_core;

use futures::Future;
use futures::stream::Stream;
use hyper::Client;
use hyper_tls::HttpsConnector;
use kitsu_io::KitsuHyperRequester;
use std::io::{self, Write};
use tokio_core::reactor::Core;

#[ignore]
#[test]
fn test_get_anime() {
    let mut core = Core::new().unwrap();

    let connector = HttpsConnector::new(1, &core.handle()).unwrap();
    let client = Client::configure()
        .connector(connector)
        .build(&core.handle());

    let runner = client.get_anime(1).unwrap().and_then(|res| {
        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_| {
        println!("Done")
    });

    core.run(runner).unwrap();
}

#[ignore]
#[test]
fn test_get_manga() {
    let mut core = Core::new().unwrap();

    let connector = HttpsConnector::new(1, &core.handle()).unwrap();
    let client = Client::configure()
        .connector(connector)
        .build(&core.handle());

    let runner = client.get_manga(1).unwrap().and_then(|res| {
        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_| {
        println!("Done")
    });

    core.run(runner).unwrap();
}

#[ignore]
#[test]
fn test_get_user() {
    let mut core = Core::new().unwrap();

    let connector = HttpsConnector::new(1, &core.handle()).unwrap();
    let client = Client::configure()
        .connector(connector)
        .build(&core.handle());

    let runner = client.get_user(1).unwrap().and_then(|res| {
        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_| {
        println!("Done")
    });

    core.run(runner).unwrap();
}

#[ignore]
#[test]
fn test_search_anime() {
    let mut core = Core::new().unwrap();

    let connector = HttpsConnector::new(1, &core.handle()).unwrap();
    let client = Client::configure()
        .connector(connector)
        .build(&core.handle());

    let runner = client.search_anime(|f| f.filter("text", "non non biyori"))
        .unwrap().and_then(|res| {
            res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
            })
        }).map(|_| {
            println!("Done")
        });

    core.run(runner).unwrap();
}

#[ignore]
#[test]
fn test_search_manga() {
    let mut core = Core::new().unwrap();

    let connector = HttpsConnector::new(1, &core.handle()).unwrap();
    let client = Client::configure()
        .connector(connector)
        .build(&core.handle());

    let runner = client.search_manga(|f| f.filter("text", "orange"))
        .unwrap().and_then(|res| {
            res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
            })
        }).map(|_| {
            println!("Done")
        });

    core.run(runner).unwrap();
}

#[ignore]
#[test]
fn test_search_users() {
    let mut core = Core::new().unwrap();

    let connector = HttpsConnector::new(1, &core.handle()).unwrap();
    let client = Client::configure()
        .connector(connector)
        .build(&core.handle());

    let runner = client.search_users(|f| f.filter("name", "vikhyat"))
        .unwrap().and_then(|res| {
            res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
            })
        }).map(|_| {
            println!("Done")
        });

    core.run(runner).unwrap();
}
