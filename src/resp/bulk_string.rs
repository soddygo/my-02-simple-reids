use std::ops::Deref;
use std::str::FromStr;

use bytes::{Buf, BytesMut};

use crate::resp::{parse_length, parse_length_for_nullable, CRLF_LENGTH};
use crate::{RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, PartialEq)]
pub struct BulkString(pub(crate) Vec<u8>, pub(crate) bool);
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        //$<length>\r\n<data>\r\n

        if self.1 {
            //nill bulk string
            b"$-1\r\n".to_vec()
        } else {
            let mut buf = Vec::with_capacity(self.0.len() + 16);

            buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());

            buf.extend_from_slice(&self);

            buf.extend_from_slice(b"\r\n");

            buf
        }
    }
}

impl RespDecode for BulkString {
    const PREFIX: &'static str = "$";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        // $-1\r\n  ,空值兼容处理，判断长度len是否-1
        let (end, len) = parse_length_for_nullable(buf, Self::PREFIX)?;
        let remained = &buf[end + CRLF_LENGTH..];

        if len == -1 {
            Ok(BulkString::nill_new())
        } else {
            if remained.len() < len as usize + CRLF_LENGTH {
                return Err(RespError::NotComplete);
            }

            buf.advance(end + CRLF_LENGTH);

            let data = buf.split_to(len as usize + CRLF_LENGTH);
            Ok(BulkString::new(data[..len as usize].to_vec()))
        }
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        Ok(end + CRLF_LENGTH + len + CRLF_LENGTH)
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into(), false)
    }
    //nill 类型类型
    pub fn nill_new() -> Self {
        BulkString(vec![], true)
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
        Ok(BulkString::new(s.as_bytes().to_vec()))
    }
}

impl From<String> for BulkString {
    fn from(value: String) -> Self {
        BulkString::new(value.into_bytes())
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(value: &[u8; N]) -> Self {
        BulkString::new(value.to_vec())
    }
}
