use nom::{
    IResult,
    bytes::complete::{tag, take, take_until},
    combinator::{cut, fail, peek},
    error::context,
    multi::count,
    number::complete::le_f32,
    sequence::terminated,
};

// nom helpers
pub type NomResult<'a, T> = IResult<&'a [u8], T>;

pub fn null_string(i: &[u8]) -> NomResult<'_, &[u8]> {
    let (i, string) = peek(terminated(take_until("\x00"), tag("\x00")))(i)?;
    take(string.len() + 1)(i)
}

pub fn take_point_float(i: &[u8]) -> NomResult<'_, Vec<f32>> {
    count(le_f32, 3)(i)
}

pub fn nom_fail<T>(s: impl AsRef<str> + Into<String>) -> NomResult<'static, T> {
    // bullshit
    let leak = s.into().leak();
    context(leak, cut(fail))(leak.as_bytes())
}
