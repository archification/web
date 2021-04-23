#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

mod other;
mod downloads;
mod catch;

fn main() {
    rocket::ignite()
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .mount("/",
    routes![
        other::favicon,
        other::index,
        other::safe_files,
        downloads::rainbow,
        downloads::stuff,
        downloads::potatos,
        other::unsafe_files,
    ])
    .attach(Template::fairing())
    .register(catchers![catch::not_found])
    .launch();
}
