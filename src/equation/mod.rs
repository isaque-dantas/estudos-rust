pub mod builder;
pub mod serializer;
pub mod repository;

#[derive(Debug, PartialEq)]
pub struct EquationValue {
    pub coefficient: u8,
    pub has_variable: bool,
}

#[derive(Debug, PartialEq)]
pub enum EquationMember {
    Value(EquationValue),
    EqualitySign,
}

pub struct RawEquation {
    pub id: usize,
    pub members: Vec<EquationMember>,
    pub answer: f32,
}

#[derive(serde::Serialize)]
pub struct EquationDTO {
    pub id: i32,
    pub content: String,
}
