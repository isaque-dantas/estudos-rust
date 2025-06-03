#[macro_use]
extern crate rocket;

pub mod equation_builder;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    equation_builder::build_random();

    rocket::build().mount("/", routes![index])
}
