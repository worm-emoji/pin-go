#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
use regex::Regex;
use reqwest;
use rocket::http::hyper::header::Location;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::mem;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
struct PinboardItem {
    href: String,
    tags: String,
}

struct Mappings {
    pinboard: HashMap<String, String>,
}

struct SharedState {
    mappings: std::sync::Arc<std::sync::Mutex<Mappings>>,
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

fn get_pinboard() -> Result<Mappings, reqwest::Error> {
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
    println!("Fetched and mapped Pinboard bookmarks.");
    return Ok(Mappings { pinboard: mapping });
}

#[get("/refresh")]
fn refresh(state: State<SharedState>) -> String {
    let new_mappings = get_pinboard().unwrap();
    let mut mappings = state.mappings.lock().unwrap();
    // Get a memory lock and replace the mappings with new ones
    mem::replace(&mut *mappings, new_mappings);
    return String::from("Refreshed.");
}

#[get("/<link>")]
fn golink(link: String, state: State<SharedState>) -> RawRedirect {
    let mappings = state.mappings.clone();
    let unlocked = mappings.lock().unwrap();
    match unlocked.pinboard.get(&link) {
        Some(url) => RawRedirect((), Location(url.to_string())),
        None => RawRedirect((), Location(String::from("/not/found"))),
    }
}

fn main() {
    let mapping = Arc::new(Mutex::new(get_pinboard().expect("Err")));
    rocket::ignite()
        .mount("/", routes![golink, refresh])
        .manage(SharedState { mappings: mapping })
        .launch();
}
