use nom::branch::alt;
use nom::bytes::complete::{tag, };
use nom::character::complete::multispace0;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct ConstantDeclarations {
    pub constants: Vec<String>,
}

pub fn constant(input: &str) -> IResult<&str, ConstantDeclarations> {
    let (input, _) = multispace0(input)?;
    let (input, _) = alt((tag("CONSTANT"), tag("CONSTANTS")))(input)?;
    Ok((input, ConstantDeclarations{ constants: vec![] }))
}

#[cfg(test)]
mod test {
    use crate::keyword::constant::{constant, ConstantDeclarations};

    #[test]
    pub fn singular_valid() {
        let cases = [
            ("CONSTANT", ("", ConstantDeclarations{ constants: vec![] })),
            (" CONSTANT", ("", ConstantDeclarations{ constants: vec![] })),
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
    pub fn plural_valid() {

    }
}