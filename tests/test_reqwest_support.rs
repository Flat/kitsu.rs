#![cfg(feature = "reqwest-support")]

extern crate kitsu_io;
extern crate reqwest;

use kitsu_io::KitsuReqwestRequester;
use reqwest::Client;

#[ignore]
#[test]
fn test_get_anime() {
    let client = Client::new();
    let res = client.get_anime(1).unwrap();

    assert_eq!(res.data.id, "1");
}

#[ignore]
#[test]
fn test_get_manga() {
    let client = Client::new();
    let res = client.get_manga(1).unwrap();

    assert_eq!(res.data.id, "1");
}

#[ignore]
#[test]
fn test_get_user() {
    let client = Client::new();
    let res = client.get_user(1).unwrap();

    assert_eq!(res.data.id, "1");
}

#[ignore]
#[test]
fn test_search_anime() {
    let client = Client::new();
    let res = client.search_anime(|f| f.filter("text", "non non biyori")).unwrap();

    assert!(res.data.len() > 0);
}

#[ignore]
#[test]
fn test_search_manga() {
    let client = Client::new();
    let res = client.search_manga(|f| f.filter("text", "orange")).unwrap();

    assert!(res.data.len() > 0);
}

#[ignore]
#[test]
fn test_search_users() {
    let client = Client::new();
    let res = client.search_users(|f| f.filter("name", "vikhyat")).unwrap();

    assert!(res.data.len() > 0);
}
