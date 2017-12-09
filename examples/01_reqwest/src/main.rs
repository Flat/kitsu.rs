extern crate kitsu_io;
extern crate reqwest;

use kitsu_io::KitsuReqwestRequester;
use reqwest::Client;
use std::io::{self, Write};

fn main() {
    // Create the reqwest Client.
    let client = Client::new();

    // Read an anime name to search for from the users input.
    let mut input = String::new();
    print!("Enter an anime name to search for:\n>");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input_trimmed = input.trim();

    // Search for the anime.
    let anime = client.search_anime(|f| f.filter("text", input_trimmed))
        .expect("Error searching for anime");

    // Print out the response of the request.
    if let Some(ref picked) = anime.data.first() {
        let title = &picked.attributes.canonical_title;

        if let Some(ref rating) = picked.attributes.average_rating {
            println!("Found Anime: {} - {}", title, rating);
        } else {
            println!("Found Anime: {} - ??", title);
        }
    } else {
        println!("No Anime Found.");
    }
}
