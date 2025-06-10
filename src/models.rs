use crate::equation::EquationDTO;
use crate::schema::equations;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = equations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Equation {
    pub id: i32,
    pub content: String,
    pub answer: f32,
}

impl Equation {
    pub fn to_dto(self) -> EquationDTO {
        EquationDTO {
            id: self.id,
            content: self.content,
        }
    }

    pub fn is_answer_correct(&self, answer: f32) -> bool {
        format!("{:.2}", self.answer) == format!("{:.2}", answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_answer_correct() {
        let equation = Equation {id: 0, content: String::new(), answer: 10.3221346};
        let correct_answer: f32 = 10.32;
        
        assert!(equation.is_answer_correct(correct_answer));
    }
}

#[derive(Selectable, Insertable)]
#[diesel(table_name = equations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EquationForm {
    pub content: String,
    pub answer: f32,
}
