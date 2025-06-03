use diesel::prelude::*;
use crate::schema::equations;

#[derive(Queryable)]
#[diesel(table_name = equations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Equation {
    pub id: usize,
    pub content: String,
    pub answer: f32
}

#[derive(Selectable, Insertable)]
#[diesel(table_name = equations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EquationForm {
    pub content: String,
    pub answer: f32
}
