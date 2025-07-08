#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("html/index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/assets", FileServer::from("assets"))
        .attach(Template::fairing())
}
