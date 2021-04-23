use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::response::NamedFile;

#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").ok()
}

#[get("/")]
pub fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

#[get("/safe")]
pub fn safe_files() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("safe", context)
}

#[get("/unsafe")]
pub fn unsafe_files() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("unsafe", context)
}
