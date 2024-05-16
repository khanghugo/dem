use nom::{
    bytes::complete::{tag, take, take_until},
    combinator::peek,
    sequence::terminated,
    IResult,
};

// nom helpers
pub type Result<'a, T> = IResult<&'a [u8], T>;

pub fn null_string(i: &[u8]) -> Result<&[u8]> {
    let (i, string) = peek(terminated(take_until("\x00"), tag("\x00")))(i)?;
    take(string.len() + 1)(i)
}
