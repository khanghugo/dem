use nom::{
    IResult, Parser,
    bytes::complete::{tag, take, take_until},
    combinator::{fail, peek},
    multi::count,
    number::complete::le_f32,
    sequence::terminated,
};

// nom helpers
pub type NomResult<'a, T> = IResult<&'a [u8], T>;

pub fn null_string(i: &[u8]) -> NomResult<'_, &[u8]> {
    let (i, string) = peek(terminated(take_until("\x00"), tag("\x00"))).parse(i)?;
    take(string.len() + 1).parse(i)
}

pub fn take_point_float(i: &[u8]) -> NomResult<'_, Vec<f32>> {
    count(le_f32, 3).parse(i)
}

pub fn nom_fail<T>(s: impl AsRef<str> + Into<String>) -> NomResult<'static, T> {
    // bullshit
    let leak = s.into().leak();
    fail().parse(leak.as_bytes())
}
