use std::fs::File;
use rocket::response::Stream;

#[get("/static/rainbow.exe")]
pub fn rainbow() -> Option<Stream<File>> {
    File::open("rainbow.exe").map(|file| Stream::from(file)).ok()
}

#[get("/static/stuff.zip")]
pub fn stuff() -> Option<Stream<File>> {
    File::open("stuff.zip").map(|file| Stream::from(file)).ok()
}

#[get("/static/potatos.zip")]
pub fn potatos() -> Option<Stream<File>> {
    File::open("stuff.zip").map(|file| Stream::from(file)).ok()
}
