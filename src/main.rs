#![feature(plugin)]
#![plugin(rocket_codegen)]

use std::path::{Path, PathBuf};

extern crate rocket;
use rocket::response::NamedFile;

#[get("/files/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    match file.extension() {
        Some(ext) if ext == "json" => return None,
        _ => {}
    }

    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![files]).launch();
}