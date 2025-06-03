pub mod equation_builder {
    use rand::Rng;
    #[derive(Debug)]
    pub struct EquationValue {
        coefficient: u8,
        has_variable: bool,
    }
    
    pub enum EquationMember {
        Value(EquationValue),
        EqualitySign
    }

    pub fn build() -> Vec<EquationMember> {
        let mut rng = rand::rng();
        let quantity = rng.random_range(2..=5);

        let coefficients = get_coefficients(quantity);
        let has_variables = get_has_variables(quantity);
        
        let mut members: Vec<EquationMember> = Vec::new();
        
        for i in 0..quantity {
            let coefficient = coefficients[i as usize];
            let has_variable = has_variables[i as usize];
            members.push(
                EquationMember::Value(
                    EquationValue {coefficient, has_variable}
                )
            );
        }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
    }
}
