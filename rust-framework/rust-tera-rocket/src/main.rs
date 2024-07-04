#[macro_use] exten crate rocket;

use rocket_dyn_templates:Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let title = "Tera with rocket";
    let message = "Howdy, from Tera";
    let mut context = HashMap::new();
    context.insert("title", title);
    context.insert("message", message);

    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}
