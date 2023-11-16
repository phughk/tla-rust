use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct ConstantDeclarations {
    pub constants: Vec<String>,
}

pub fn constant(input: &str) -> IResult<&str, ConstantDeclarations> {
    let (remainder, _) = alt((tag("CONSTANT"), tag("CONSTANTS")))(input)?;
    Ok((remainder, ConstantDeclarations{ constants: vec![] }))
}

#[cfg(test)]
mod test {
    use crate::keyword::constant::{constant, ConstantDeclarations};

    #[test]
    pub fn singular_empty() {
        let res = constant("CONSTANT");
        assert!(res.is_ok());
        let (remainder, const_declaration) = res.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(const_declaration, ConstantDeclarations{ constants: vec![] })
    }

    #[test]
    pub fn singular_single() {
    }

    #[test]
    pub fn singular_many() {

    }

    #[test]
    pub fn plural_single() {

    }

    #[test]
    pub fn plural_many() {

    }
}