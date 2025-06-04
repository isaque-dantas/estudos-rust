use crate::schema::equations;
use diesel::prelude::*;
use crate::equation::EquationDTO;

#[derive(Queryable)]
#[diesel(table_name = equations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Equation {
    pub id: i32,
    pub content: String,
    pub answer: f32,
}

impl Equation {
    pub fn to_dto(self) -> EquationDTO {
        EquationDTO {id: self.id, content: self.content}
    }
}

#[derive(Selectable, Insertable)]
#[diesel(table_name = equations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EquationForm {
    pub content: String,
    pub answer: f32,
}
