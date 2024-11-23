use nom::{
    branch::alt, 
    bytes::complete::{tag, take_while1}, 
    character::complete::{digit1, space0, char}, 
    combinator::{map, map_res}, 
    sequence::tuple, 
    IResult
};

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Val(f64),  // constant value - number (positive integer or positive float)
    Var(String), // variable name
    Exp(Box<Unit>, String, Box<Unit>) // expression, eg :size / 3
}

fn parse_number(input: &str) -> IResult<&str, Unit> {
    alt((
        // parse float
        map_res(
            tuple((
                digit1,
                char('.'), 
                digit1,
            )),
            |(integer, _, fraction): (&str, char, &str)| {
                let value: f64 = format!("{}.{}", integer, fraction).parse().unwrap();
                Ok::<_, nom::Err<(&str, nom::error::ErrorKind)>>(Unit::Val(value))
            },
        ),
        // parse positive integer
        map_res(digit1, |digits: &str| {
            let value: f64 = digits.parse().unwrap();
            Ok::<_, nom::Err<(&str, nom::error::ErrorKind)>>(Unit::Val(value))
        }),
    ))(input)
}

fn parse_variable(input: &str) -> IResult<&str, Unit> {
    let (input, _) = nom::bytes::complete::tag(":")(input)?;
    map(take_while1(|c: char| c.is_alphanumeric() || c == '_'), |s: &str| {
        Unit::Var(s.to_string())
    })(input)
}

// todo: obsługa zagnieżdżonych wyrażeń
fn parse_expression(input: &str) -> IResult<&str, Unit> {
    let (input, (left, _, operator, _, right)) = tuple((
        alt((parse_number, parse_variable)),
        space0,
        alt((tag("+"), tag("-"), tag("*"), tag("/"))),
        space0,
        alt((parse_number, parse_variable)),
    ))(input)?;
    Ok((
        input,
        Unit::Exp(Box::new(left), operator.to_string(), Box::new(right)),
    ))
}

pub fn parse_unit(input: &str) -> IResult<&str, Unit> {
    alt((parse_expression, parse_number, parse_variable))(input)
}