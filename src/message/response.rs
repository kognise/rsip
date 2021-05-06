use crate::{
    common::{
        status_code::{self, StatusCode},
        version::{self, Version},
    },
    headers::{header, Headers},
    Error, NomError, SipMessage,
};
//use bytes::Bytes;
//use nom::error::VerboseError;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Response {
    pub status_code: StatusCode,
    pub version: Version,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn status_code(&self) -> &StatusCode {
        &self.status_code
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        Ok(Self {
            version: tokenizer.version.try_into()?,
            status_code: tokenizer.status_code.try_into()?,
            headers: tokenizer
                .headers
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, Error>>()?
                .into(),
            body: tokenizer.body.into(),
        })
    }
}

impl TryFrom<SipMessage> for Response {
    type Error = &'static str;

    fn try_from(sip_message: crate::SipMessage) -> Result<Self, Self::Error> {
        match sip_message {
            crate::SipMessage::Request(_) => {
                Err("Can't convert a models::SipMessage::Response into Request !")
            }
            crate::SipMessage::Response(response) => Ok(response),
        }
    }
}

impl TryFrom<&[u8]> for Response {
    type Error = Error;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        Self::parse(Tokenizer::tokenize(from)?.1)
    }
}

impl TryFrom<Vec<u8>> for Response {
    type Error = Error;

    fn try_from(from: Vec<u8>) -> Result<Self, Self::Error> {
        Self::parse(Tokenizer::tokenize(&from)?.1)
    }
}

impl TryFrom<&str> for Response {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Self::parse(Tokenizer::tokenize(from.as_bytes())?.1)
    }
}

impl TryFrom<String> for Response {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        Self::parse(Tokenizer::tokenize(&from.as_bytes())?.1)
    }
}

impl TryFrom<bytes::Bytes> for Response {
    type Error = Error;

    fn try_from(from: bytes::Bytes) -> Result<Self, Self::Error> {
        Self::parse(Tokenizer::tokenize(&from)?.1)
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for Response {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Error> {
        Self::parse(tokenizer)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tokenizer<'a> {
    pub version: version::Tokenizer<'a>,
    pub status_code: status_code::Tokenizer<'a>,
    pub headers: Vec<header::Tokenizer<'a>>,
    pub body: &'a [u8],
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
        use nom::{bytes::complete::tag, multi::many0, sequence::tuple};

        let (rem, (version, status_code)) = tuple((
            version::Tokenizer::tokenize,
            status_code::Tokenizer::tokenize_with_reason,
        ))(part)?;
        let (rem, headers) = many0(header::Tokenizer::tokenize)(rem)?;
        let (body, _) = tag("\r\n")(rem)?;

        Ok((
            &[],
            Self {
                version,
                status_code,
                headers,
                body,
            },
        ))
    }
}

/*
impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Into::<libsip::core::SipMessage>::into(self.clone())
        )
    }
}

impl Into<Bytes> for Response {
    fn into(self) -> Bytes {
        crate::SipMessage::from(self).into()
    }
}

impl TryFrom<Bytes> for Response {
    type Error = String;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        use std::convert::TryInto;

        let (_, sip_message) = libsip::parse_message::<VerboseError<&[u8]>>(&bytes.to_vec())
            .map_err(|e| e.to_string())?;

        Ok(sip_message.try_into()?)
    }
}*/
