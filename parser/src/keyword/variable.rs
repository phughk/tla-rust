use nom::branch::alt;
use nom::bytes::complete::{tag, };
use nom::character::complete::{alpha0, alpha1, char, multispace0, multispace1};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::preceded;

#[derive(Debug, PartialEq)]
pub struct VariableDeclarations<'a> {
    pub variables: Vec<&'a str>,
}

pub fn variable(input: &str) -> IResult<&str, VariableDeclarations> {
    let (input, _) = multispace0(input)?;
    let (input, _) = alt((tag("VARIABLES"), tag("VARIABLE")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, vars) = separated_list1( char(','), preceded(multispace0, alpha1))(input)?;

    Ok((input, VariableDeclarations { variables: vars }))
}

#[cfg(test)]
mod test {
    use crate::keyword::variable::{variable, VariableDeclarations};

    #[test]
    pub fn valid() {
        let cases = [
            ("VARIABLE a", ("", VariableDeclarations { variables: vec!["a"] })),
            (" \t VARIABLE b", ("", VariableDeclarations { variables: vec!["b"] })),
            ("VARIABLES c", ("", VariableDeclarations { variables: vec!["c"] })),
            (" \t VARIABLES d", ("", VariableDeclarations { variables: vec!["d"] })),
            ("VARIABLE a, e", ("", VariableDeclarations { variables: vec!["a", "e"] })),
            ("VARIABLE a, e, ", (", ", VariableDeclarations { variables: vec!["a", "e"] })),
        ];
        for (index, (input, output)) in cases.iter().enumerate() {
            println!("Case {}: input={:?}, output={:?}", index, input, output);
            let res = variable(input);
            assert!(res.is_ok());
            let (remainder, const_declaration) = res.unwrap();
            assert_eq!(remainder, output.0);
            assert_eq!(const_declaration, output.1);
        }
    }

    #[test]
    pub fn invalid() {

    }
}
