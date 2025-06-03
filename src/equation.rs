#[derive(Debug)]
#[derive(PartialEq)]
pub struct EquationValue {
    pub coefficient: u8,
    pub has_variable: bool,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum EquationMember {
    Value(EquationValue),
    EqualitySign,
}

pub struct Equation {
    pub id: usize,
    pub members: Vec<EquationMember>,
    pub answer: f64
}

#[derive(serde::Serialize)]
pub struct EquationDTO {
    pub id: usize,
    pub content: String,
}
