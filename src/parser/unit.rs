use nom::{
    branch::alt, bytes::complete::{tag, tag_no_case, take_while1}, character::complete::{alphanumeric1, char, digit1, multispace0, space0, space1}, combinator::{map, map_res, opt}, multi::many1, sequence::{delimited, preceded, tuple}, IResult
};

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Val(f64),  // constant value - number (integer or float)
    Var(String), // variable name
    Random(Box<Unit>), // random value
    Exp(Box<Unit>, String, Box<Unit>), // expression, eg :size / 3
    Pick(Vec<String>)
}


fn parse_random(input: &str) -> IResult<&str, Unit> {
    let (input, _) = alt((tag_no_case("random"), tag_no_case("rm")))(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = parse_unit(input)?;
    Ok((input, Unit::Random(Box::new(value))))
}


fn parse_pick(input: &str) -> IResult<&str, Unit> {
    let (input, _) = tag_no_case("pick")(input)?;
    let (input, _) = space1(input)?;
    let (input, list) = delimited(
        char('['), 
        many1(map(preceded(multispace0, alphanumeric1), |s: &str| s.to_string())), // Zbierz elementy jako stringi
        preceded(multispace0, char(']')),
    )(input)?;
    Ok((input, Unit::Pick(list)))
}

pub fn parse_number(input: &str) -> IResult<&str, Unit> {
    alt((
        // parse float with optional negative sign
        map_res(
            tuple((
                opt(char('-')), // Optional negative sign
                digit1,
                char('.'),
                digit1,
            )),
            |(sign, integer, _, fraction): (Option<char>, &str, char, &str)| {
                let mut value: f64 = format!("{}.{}", integer, fraction).parse().unwrap();
                if sign.is_some() {
                    value = -value;
                }
                Ok::<_, nom::Err<(&str, nom::error::ErrorKind)>>(Unit::Val(value))
            },
        ),
        // parse integer with optional negative sign
        map_res(
            tuple((opt(char('-')), digit1)),
            |(sign, digits): (Option<char>, &str)| {
                let mut value: f64 = digits.parse().unwrap();
                if sign.is_some() {
                    value = -value;
                }
                Ok::<_, nom::Err<(&str, nom::error::ErrorKind)>>(Unit::Val(value))
            },
        ),
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
    alt((parse_expression, parse_number, parse_variable, parse_random))(input)
}