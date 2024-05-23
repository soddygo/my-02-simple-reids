use std::ops::Deref;
use std::str::FromStr;

use bytes::{Buf, BytesMut};

use crate::resp::{extract_fixed_data, parse_length, CRLF_LENGTH};
use crate::{RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, PartialEq)]
pub struct BulkString(pub(crate) Vec<u8>);

#[derive(Debug, Clone, PartialEq)]
pub struct RespNullBulkString;

impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        //$<length>\r\n<data>\r\n
        let mut buf = Vec::with_capacity(self.0.len() + 16);

        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());

        buf.extend_from_slice(&self);

        buf.extend_from_slice(b"\r\n");

        buf
    }
}

impl RespDecode for BulkString {
    const PREFIX: &'static str = "$";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let remained = &buf[end + CRLF_LENGTH..];
        if remained.len() < len + CRLF_LENGTH {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LENGTH);

        let data = buf.split_to(len + CRLF_LENGTH);
        Ok(BulkString::new(data[..len].to_vec()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        Ok(end + CRLF_LENGTH + len + CRLF_LENGTH)
    }
}

impl RespDecode for RespNullBulkString {
    const PREFIX: &'static str = "$";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "$-1\r\n", "NullBulkString")?;
        Ok(RespNullBulkString)
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(5)
    }
}

impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into())
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for BulkString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BulkString(s.as_bytes().to_vec()))
    }
}

impl From<String> for BulkString {
    fn from(value: String) -> Self {
        BulkString(value.into_bytes())
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(value: &[u8; N]) -> Self {
        BulkString(value.to_vec())
    }
}
