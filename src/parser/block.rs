use nom::{
    branch::alt, 
    bytes::complete::{tag, tag_no_case, take_while1}, 
    character::complete::{char, multispace0, space0, space1}, 
    combinator::{map, map_res}, multi::{many1, separated_list0}, 
    sequence::{delimited, preceded, tuple}, 
    IResult
};

use super::command::{parse_command, Command};
use super::unit::{Unit, parse_unit};

#[derive(Debug, PartialEq, Clone)]
pub struct Condition {
    pub left: Unit,
    pub operator: String,
    pub right: Unit,
}

pub fn parse_condition(input: &str) -> IResult<&str, Condition> {

    fn parse_operator(input: &str) -> IResult<&str, String> {
        alt((
            map(tag("<"), |s: &str| s.to_string()),
            map(tag(">"), |s: &str| s.to_string()),
            map(tag("=="), |s: &str| s.to_string()),
        ))(input)
    }

    let (input, (left, operator, right)) = tuple((
        preceded(space0, parse_unit),
        preceded(space0, parse_operator),
        preceded(space0, parse_unit),
    ))(input)?;
    
    Ok((input, Condition { left, operator, right }))
}

#[derive(Debug, PartialEq, Clone)]
pub enum Block {
    Single(Command), // instruction
    Repeat(Unit, Vec<Block>), // iterations, instructions
    If(Condition, Vec<Block>), // condition, instructions
    Function(String, Vec<String>, Vec<Block>), // name, params' names, instructions <- function definition
    Call(String, Vec<Unit>) // name, args <- function call
}

fn parse_repeat(input: &str) -> IResult<&str, Block> {
    let (input, _) = tag_no_case("repeat")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = parse_unit(input)?;
    let (input, _) = space1(input)?;
    let (input, commands) = delimited(
        char('['), 
        many1(preceded(multispace0, parse_block)),
        preceded(multispace0, char(']')),
    )(input)?;
    Ok((input, Block::Repeat(times, commands)))
}

fn parse_if(input: &str) -> IResult<&str, Block> {
    let (input, _) = tag_no_case("if")(input)?;
    let (input, _) = space1(input)?;
    let (input, condition) = parse_condition(input)?;
    let (input, _) = space1(input)?;
    let (input, commands) = delimited(
        char('['), 
        many1(preceded(multispace0, parse_block)),
        preceded(multispace0, char(']')),
    )(input)?;
    Ok((input, Block::If(condition, commands)))
}

fn parse_function(input: &str) -> IResult<&str, Block> {

    fn parse_params(input: &str) -> IResult<&str, Vec<String>> {
        separated_list0(
            space1, 
            preceded(
                char(':'),
                map(take_while1(char::is_alphanumeric), |s: &str| s.to_string())
            )
        )(input)
    }

    let (input, _) = tag_no_case("to ")(input)?; 
    let (input, name) = map(take_while1(char::is_alphanumeric), |s: &str| s.to_string())(input)?; 
    let (input, _) = space1(input)?; 
    let (input, params) = parse_params(input)?;
    let (input, commands) = many1(preceded(multispace0, parse_block))(input)?;
    let (input, _) = preceded(multispace0, tag_no_case("end"))(input)?;

    Ok((input, Block::Function(name, params, commands)))
}

fn parse_call(input: &str) -> IResult<&str, Block> {

    fn parse_args(input: &str) -> IResult<&str, Vec<Unit>> {
        separated_list0(
            space1, 
            parse_unit
        )(input)
    }

    let reserved_keywords = ["to", "repeat", "if", "end", "pick", "setcolor"]; // illegal keywords
    let (input, name) = map_res(
        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
        |s: &str| {
            if reserved_keywords.contains(&s) {
                Err("Reserved keyword")
            } else {
                Ok(s.to_string())
            }
        },
    )(input)?;
    let (input, _) = space0(input)?; 
    let (input, args) = parse_args(input)?; 

    Ok((input, Block::Call(name, args)))
}


pub fn parse_block(input: &str) -> IResult<&str, Block> {
    preceded(
        multispace0, // ignoruje białe znaki przed blokiem
        alt((
            map(parse_command, Block::Single),
            parse_repeat,
            parse_if,
            parse_function,
            parse_call,
        )),
    )(input)
}
