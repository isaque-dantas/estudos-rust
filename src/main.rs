#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;
use equation::EquationDTO;

mod equation;
pub mod models;
pub mod schema;

#[get("/equation")]
fn get_equation() -> Result<Json<EquationDTO>, Status> {
    let equation = equation::builder::build_random();
    let equation_content = equation::serializer::serialize_members(&equation.members);
    
    let saved_equation = equation::repository::save(
        equation_content.as_str(),
        equation.answer
    );
    
    match saved_equation {
        Some(eq) => Ok(Json(eq.to_dto())),
        None => Err(Status::InternalServerError) 
    }
}

// #[catch(500)]
// fn internal_error(req: &Request) -> String {
//     String::from("There was an error in the application. Contact support for more details.")
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/", routes![get_equation])
        // .register("/", catchers![internal_error])
}
