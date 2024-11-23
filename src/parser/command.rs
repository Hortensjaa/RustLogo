use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::space1,
    IResult,
};

use super::unit::{parse_unit, Unit};

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Forward(Unit),
    Left(Unit),
    Right(Unit),
    Back(Unit),
    Wait(Unit),
    ClearScreen(),
    PenUp(),
    PenDown()
}

fn parse_forward(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag_no_case("forward"), tag_no_case("fd")))(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = parse_unit(input)?;
    Ok((input, Command::Forward(value)))
}

fn parse_left(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag_no_case("left"), tag_no_case("lt")))(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = parse_unit(input)?;
    Ok((input, Command::Left(value)))
}

fn parse_right(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag_no_case("right"), tag_no_case("rt")))(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = parse_unit(input)?;
    Ok((input, Command::Right(value)))
}

fn parse_back(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag_no_case("back"), tag_no_case("bk")))(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = parse_unit(input)?;
    Ok((input, Command::Back(value)))
}

fn parse_wait(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag_no_case("wait"), tag_no_case("wt")))(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = parse_unit(input)?;
    Ok((input, Command::Wait(value)))
}

fn parse_clearscreen(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("clearscreen")(input)?;
    Ok((input, Command::ClearScreen()))
}

fn parse_penup(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag_no_case("penup"), tag_no_case("pu")))(input)?;
    Ok((input, Command::PenUp()))
}

fn parse_pendown(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag_no_case("pendown"), tag_no_case("pd")))(input)?;
    Ok((input, Command::PenDown()))
}

pub fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((
        parse_forward, 
        parse_left, 
        parse_right, 
        parse_back,
        parse_clearscreen,
        parse_wait,
        parse_pendown,
        parse_penup
    ))(input)
}


