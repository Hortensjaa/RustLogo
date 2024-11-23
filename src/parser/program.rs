use nom::{
    multi::many1,
    character::complete::multispace0,
    sequence::preceded,
    IResult,
};

use super::block::{parse_block, Block};

pub fn parse_program(input: &str) -> IResult<&str, Vec<Block>> {
    many1(preceded(multispace0, parse_block))(input)
}
