#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate harsh;

use std::sync::RwLock;
use rocket::State;

mod repo;
mod shortener;
use repo::Repository;

#[get("/<id>")]
fn lookup(rep: State<RwLock<Repository>>, id: &str) -> String {
    match rep.read().unwrap().lookup(id) {
        Some(url) => format!("id {} redirects to {}", id, url),
        _ => format!("id {} was not found", id)
    }
}

#[get("/<url>")]
fn store(rep: State<RwLock<Repository>>, url: &str) -> String {
    let mut rep = rep.write().unwrap();
    rep.store(url);
    let id = rep.store(&url);
   format!("url {} was stored with id {}", url, id)
}

fn main() {
    rocket::ignite().manage(RwLock::new(Repository::new()))
                    .mount("/id", routes![lookup])
                    .mount("/store", routes![store])
                    .launch();
}
