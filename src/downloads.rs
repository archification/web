use std::fs::File;
use rocket::response::Stream;

#[get("/static/rainbow.exe")]
pub fn rainbow() -> Option<Stream<File>> {
    File::open("rainbow.exe").map(|file| Stream::from(file)).ok()
}
