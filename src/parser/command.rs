use nom::{
    branch::alt, bytes::complete::{tag, tag_no_case, take_while1}, character::complete::{alpha0, alpha1, alphanumeric1, char, multispace0, multispace1, space1}, combinator::map, multi::{many1, separated_list1}, sequence::{delimited, preceded}, IResult
};
use std::collections::HashSet;

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
    PenDown(),
    SetColor(String),
    SetColorPick(Vec<String>),
    SetTurtle(Unit),
    Stop(),
    HideTurtle(),
    ShowTurtle(),
    Window()
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

fn parse_stop(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag_no_case("stop"), tag_no_case("sp")))(input)?;
    Ok((input, Command::Stop()))
}

fn parse_showturtle(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("showturtle")(input)?;
    Ok((input, Command::ShowTurtle()))
}

fn parse_hideturtle(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("hideturtle")(input)?;
    Ok((input, Command::HideTurtle()))
}

fn parse_window(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("window")(input)?;
    Ok((input, Command::Window()))
}

fn parse_setcolor(input: &str) -> IResult<&str, Command> {
    let allowed_colors: HashSet<&str> = [
        "black", "blue", "green", "cyan", "red", "magenta", 
        "yellow", "white", "brown", "tan", "aqua", "salmon", 
        "purple", "orange", "gray"
    ].iter().cloned().collect();

    let (input, _) = alt((tag_no_case("setcolor"), tag_no_case("sc")))(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("\"")(input)?;
    
    let (input, color) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;

    if allowed_colors.contains(color) {
        Ok((input, Command::SetColor(color.to_string())))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))
    }
}

fn parse_setcolorpick(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag_no_case("setcolor"), tag_no_case("sc")))(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag_no_case("pick")(input)?;
    let (input, _) = space1(input)?;
    let (input, list) = delimited(
        char('['), 
        many1(preceded(multispace0, alpha1)),
        preceded(multispace0, char(']')),
    )(input)?;
    let strings = list.into_iter().map(|s| s.to_string()).collect();
    Ok((input, Command::SetColorPick(strings))) 
}

fn parse_setturtle(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag_no_case("setturtle")(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = parse_unit(input)?;
    Ok((input, Command::SetTurtle(value)))
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
        parse_penup,
        parse_setcolorpick,
        parse_setcolor,
        parse_stop,
        parse_showturtle,
        parse_hideturtle,
        parse_window,
        parse_setturtle,
    ))(input)
}


