use crate::error::ParseJwtTokenError;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    error::Error,
    sequence::pair,
    IResult,
};
use std::num::ParseIntError;

pub fn length(input: &str) -> Result<u64, ParseIntError> {
    let res: IResult<_, _, Error<_>> = pair(tag("Length = "), digit1)(input);

    let Ok((rest, (_, len))) = res else {
        return "NaN".parse(); // 不管是什么错误，直接丢这个出去
    };

    // 必须保证整行都有效
    if rest.is_empty() { len } else { "NaN" }.parse()
}

pub fn token(input: &str) -> Result<&str, ParseJwtTokenError> {
    let res: IResult<_, _, Error<_>> = pair(
        tag("Token = "),
        take_while1(|c: char| c.is_alphanumeric() || matches!(c, '-' | '_' | '.')),
    )(input);

    res.map_err(|_| ParseJwtTokenError)
        .and_then(|(rest, (_, token))| {
            // 必须保证整行都有效
            if rest.is_empty() {
                Ok(token)
            } else {
                Err(ParseJwtTokenError)
            }
        })
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_length() {
        use super::length;

        assert_eq!(length("Length = 114514"), Ok(114514));
        assert!(length("Length = 114NaN").is_err());
        assert!(length("Length = NaN").is_err());
        assert!(length("Length = ").is_err());
    }

    #[test]
    fn test_token() {
        use super::token;

        let mock_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.\
            eyJleHAiOjE2NzE5NjIxMDksInVzZXJuYW1lIjoiQWRtaW4ifQ.\
            bZHwalBNOiKbCfnw-QxbToDBg75lZiVBHdVsF3ilxKo";
        let line = format!("Token = {mock_token}");

        assert_eq!(token(&line), Ok(mock_token));
        assert!(token("Token = 114514=+/aaaaa").is_err());
    }
}
