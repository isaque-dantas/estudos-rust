#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

pub mod equation;
pub mod models;
pub mod schema;

#[get("/equation")]
fn get_equation() -> Json<equation::EquationDTO> {
    let equation = equation::builder::build_random();
    let serialized = equation::serializer::serialize(&equation);
    
    equation::repository::save(
        serialized.content.as_str(),
        equation.answer as f32
    );
    
    Json(serialized)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/", routes![get_equation])
}
