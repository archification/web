#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate multipart;

use rocket_contrib::templates::Template;

mod other;
mod downloads;
mod catch;

fn main() {
    rocket::ignite()
    .mount("/", routes![
        other::favicon,
        other::index,
        other::safe_files,
        downloads::rainbow,
        other::unsafe_files,
        other::hello,
        other::world,
    ])
    .attach(Template::fairing())
    .register(catchers![catch::not_found])
    .launch();
}
