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
    Template::render("safe", context)
}

#[get("/world")]
pub fn world() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>/<cool>", rank = 2)]
pub fn hello(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}
