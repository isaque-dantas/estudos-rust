use rand::prelude::*;

mod equation_builder {
    use rand::Rng;

    fn build() -> String {
        let mut rng = rand::rng();
        let members_quantity = rng.random_range(2..=5);
        
        let mut members: Vec<u8> = Vec::new();
        
        for _ in 0..members_quantity {
            members.push(rng.random_range(1..=16));
        }
        
        String::new()
    }
}