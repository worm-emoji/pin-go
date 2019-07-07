#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
use regex::Regex;
use reqwest;
use rocket::http::hyper::header::Location;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Deserialize)]
struct PinboardItem {
    href: String,
    tags: String,
}

// Rocket's Redirect::to method isn't very good, and doesn't support things
// like anchor tags
#[derive(Responder)]
#[response(status = 303)]
struct RawRedirect((), Location);

fn tag_to_golink(tag: &str) -> Result<String, &str> {
    lazy_static! {
        static ref TAG_REGEX: Regex = Regex::new(r"go:(\S+)").unwrap();
    }

    match TAG_REGEX.captures_iter(&tag).nth(0) {
        Some(value) => return Ok(String::from(&value[1])),
        _ => return Err("No match"),
    };
}

fn get_pinboard() -> Result<HashMap<String, String>, reqwest::Error> {
    let url = format!(
        "https://api.pinboard.in/v1/posts/all?auth_token={}&format=json&tag=go",
        env::var("PINBOARD").unwrap()
    );
    let result: Vec<PinboardItem> = reqwest::get(&url)?.json()?;
    let mut mapping: HashMap<String, String> = HashMap::new();
    for r in &result {
        match tag_to_golink(&r.tags) {
            Ok(golink) => {
                mapping.insert(String::from(golink), String::from(&r.href));
            }
            _ => {}
        }
    }
    return Ok(mapping);
}

#[get("/<link>")]
fn golink(link: String) -> RawRedirect {
    let mapping = get_pinboard().expect("Error fetching pinboard mapping");
    match mapping.get(&link) {
        Some(url) => {
            println!("{}", &url);
            return RawRedirect((), Location(url.to_string()));
        }
        None => return RawRedirect((), Location(String::from("/not/found"))),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![golink]).launch();
}
