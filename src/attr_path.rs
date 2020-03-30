use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::streaming::{take_till1, take_while},
    combinator::{complete, not},
    error::ErrorKind,
    multi::separated_nonempty_list,
    sequence::{preceded, terminated},
    IResult,
};

fn quote(input: &str) -> IResult<&str, &str> {
    tag("\"")(input)
}

fn quoted_string(input: &str) -> IResult<&str, &str> {
    preceded(quote, terminated(take_while(|c| c != '"'), quote))(input)
}

fn parse_attr_path(input: &str) -> Result<Vec<&str>, nom::Err<(&str, ErrorKind)>> {
    complete(separated_nonempty_list(
        tag("."),
        alt((quoted_string, take_till1(|c| c == '.'))),
    ))(input)
    .map(|(_, res)| res)
}
