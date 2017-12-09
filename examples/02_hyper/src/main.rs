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

fn main() {
    // Read an anime name to search for from the users input.
    let mut input = String::new();
    print!("Enter an anime name to search for:\n>");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input_trimmed = input.trim();

    // Create the core and client which will be uesd to search.
    let mut core = Core::new().expect("Error creating reactor core");

    let connector = HttpsConnector::new(1, &core.handle())
        .expect("Error creating connector");
    let client = Client::configure()
        .connector(connector)
        .build(&core.handle());

    // Search for the anime and return the response.
    let runner = client.search_anime(|f| f.filter("text", input_trimmed))
        .expect("Error making request")
        .and_then(|res| {
            res.body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
        }).map(|_| {
            println!("\n\nDone")
        });

    core.run(runner).expect("Error running core");
}
