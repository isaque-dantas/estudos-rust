#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

pub mod equation_builder;
pub mod equation_serializer;
pub mod equation;

#[get("/equation")]
fn get_equation() -> Json<equation::EquationDTO> {
    let equation = equation_builder::build_random();
    Json(equation_serializer::serialize(&equation))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/", routes![get_equation])
}
