use crate::equation::Equation;
use crate::equation::EquationDTO;
use crate::equation::EquationMember;

pub fn serialize(equation: &Equation) -> EquationDTO {
    EquationDTO {
        id: equation.id,
        content: serialize_members(&equation.members),
    }
}

pub fn serialize_members(members: &Vec<EquationMember>) -> String {
    let mut serialized = String::new();

    for (i, member) in members.iter().enumerate() {
        match member {
            EquationMember::Value(v) => {
                if i != 0 && members[i - 1] != EquationMember::EqualitySign {
                    serialized.push('+');
                }

                serialized.push_str(v.coefficient.to_string().as_str());

                if v.has_variable {
                    serialized.push('x');
                }
            }
            EquationMember::EqualitySign => serialized.push('='),
        }
    }

    serialized.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let equation_members = vec![
            EquationMember::Value(EquationValue {
                coefficient: 1,
                has_variable: true,
            }),
            EquationMember::Value(EquationValue {
                coefficient: 3,
                has_variable: false,
            }),
            EquationMember::Value(EquationValue {
                coefficient: 2,
                has_variable: true,
            }),
            EquationMember::EqualitySign,
            EquationMember::Value(EquationValue {
                coefficient: 4,
                has_variable: false,
            }),
        ];

        let equation = Equation {
            id: 0,
            answer: 10.0,
            members: equation_members,
        };
        let serialized = serialize_members(&equation.members);
        let expected = String::from("1x+3+2x=4");

        assert_eq!(serialized, expected);
    }
}
