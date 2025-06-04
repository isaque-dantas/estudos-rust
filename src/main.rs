#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;
use equation::EquationDTO;
use crate::equation::EquationAnswer;

mod equation;
pub mod models;
pub mod schema;

#[get("/equation")]
fn get_equation() -> Result<Json<EquationDTO>, Status> {
    let equation = equation::builder::build_random();
    let equation_content = equation::serializer::serialize_members(&equation.members);

    let saved_equation = equation::repository::save(
        equation_content.as_str(),
        equation.answer,
    );

    match saved_equation {
        Some(eq) => Ok(Json(eq.to_dto())),
        None => Err(Status::InternalServerError)
    }
}

#[post("/equation/<id>/answer", data="<answer>")]
fn send_answer<'a>(id: i32, answer: Json<EquationAnswer>) -> Result<&'a str, Status> {
    let equation_query_result = equation::repository::get(id);
    match equation_query_result {
        Ok(eq) => {
            if eq.is_answer_correct(answer.answer) {
                return Ok("The answer is correct!");
            }
            
            Ok("The answer is wrong!")
        }
        Err(_) => Err(Status::NotFound)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/", routes![get_equation, send_answer])
    // .register("/", catchers![internal_error])
}
