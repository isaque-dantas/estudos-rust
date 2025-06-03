use rand::Rng;
use crate::equation::EquationMember;
use crate::equation::EquationValue;
use crate::equation::RawEquation;

pub fn build_random() -> RawEquation {
    let members = build_random_members();
    let answer = get_answer(&members);
    
    RawEquation {id: 0, members, answer: answer}
}

pub fn get_answer(members: &Vec<EquationMember>) -> f64 {
    10.0
}

pub fn build_random_members() -> Vec<EquationMember> {
    let mut rng = rand::rng();

    let quantity = rng.random_range(2..=5);
    let coefficients = get_coefficients(quantity);
    let has_variables = get_has_variables(quantity);
    let equality_sign_position = rng.random_range(1..quantity);

    return build_members(
        quantity,
        coefficients,
        has_variables,
        equality_sign_position as usize,
    );
}

pub fn build_members(
    quantity: u8,
    coefficients: Vec<u8>,
    has_variables: Vec<bool>,
    equality_sign_position: usize,
) -> Vec<EquationMember> {
    let mut members: Vec<EquationMember> = Vec::new();

    for i in 0..quantity {
        let coefficient = coefficients[i as usize];
        let has_variable = has_variables[i as usize];

        members.push(EquationMember::Value(EquationValue {
            coefficient,
            has_variable,
        }));
    }

    members.insert(
        equality_sign_position as usize,
        EquationMember::EqualitySign,
    );

    members
}

fn get_coefficients(quantity: u8) -> Vec<u8> {
    let mut rng = rand::rng();
    let mut coefficients: Vec<u8> = Vec::new();

    for _ in 0..quantity {
        coefficients.push(rng.random_range(1..=16));
    }

    coefficients
}

fn get_has_variables(quantity: u8) -> Vec<bool> {
    let mut rng = rand::rng();
    let mut has_variables: Vec<bool> = Vec::new();

    for _ in 0..quantity {
        has_variables.push(rng.random_bool(0.25));
    }

    has_variables
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let equation = build_members(4, vec![1, 3, 2, 4], vec![true, false, true, false], 3);
        let expected = vec![
            EquationMember::Value(EquationValue { coefficient: 1, has_variable: true }),
            EquationMember::Value(EquationValue { coefficient: 3, has_variable: false }),
            EquationMember::Value(EquationValue { coefficient: 2, has_variable: true }),
            EquationMember::EqualitySign,
            EquationMember::Value(EquationValue { coefficient: 4, has_variable: false }),
        ];
        
        assert_eq!(equation, expected);
    }

}
