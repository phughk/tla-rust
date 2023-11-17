use nom::branch::alt;
use nom::bytes::complete::{tag, };
use nom::character::complete::{alpha0, alpha1, char, multispace0, multispace1};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::preceded;

#[derive(Debug, PartialEq)]
pub struct ConstantDeclarations<'a> {
    pub constants: Vec<&'a str>,
}

pub fn constant_entry(input: &str) -> IResult<&str, String> {
    let (input, name) = alpha0(input)?;
    Ok((input, name.to_string()))
}

pub fn constant(input: &str) -> IResult<&str, ConstantDeclarations> {
    let (input, _) = multispace0(input)?;
    let (input, _) = alt((tag("CONSTANTS"), tag("CONSTANT")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, vars) = separated_list1( char(','), preceded(multispace0, alpha1))(input)?;

    Ok((input, ConstantDeclarations{ constants: vars }))
}

#[cfg(test)]
mod test {
    use crate::keyword::constant::{constant, ConstantDeclarations};

    #[test]
    pub fn valid() {
        let cases = [
            ("CONSTANT a", ("", ConstantDeclarations{ constants: vec!["a"] })),
            (" \t CONSTANT b", ("", ConstantDeclarations{ constants: vec!["b"] })),
            ("CONSTANTS c", ("", ConstantDeclarations{ constants: vec!["c"] })),
            (" \t CONSTANTS d", ("", ConstantDeclarations{ constants: vec!["d"] })),
            ("CONSTANT a, e", ("", ConstantDeclarations{ constants: vec!["a", "e"] })),
            ("CONSTANT a, e, ", (", ", ConstantDeclarations{ constants: vec!["a", "e"] })),
        ];
        for (index, (input, output)) in cases.iter().enumerate() {
            println!("Case {}: input={:?}, output={:?}", index, input, output);
            let res = constant(input);
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