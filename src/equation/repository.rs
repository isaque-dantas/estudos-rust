use crate::models::Equation;
use diesel::prelude::*;
use math_time_api::establish_connection;

pub fn save(content: &str, answer: f32) -> Option<Equation> {
    let connection = &mut establish_connection();
    use crate::models::EquationForm;
    use crate::schema::equations;

    let equation = EquationForm {
        content: content.to_string(),
        answer,
    };

    match diesel::insert_into(equations::table)
        .values(equation)
        .returning((equations::id, equations::content, equations::answer))
        .get_result::<(i64, &str, f64)>(connection)
    {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}
