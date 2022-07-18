#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};

use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("static")))
}
