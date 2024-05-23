use crate::{RespDecode, RespEncode, RespError};
use bytes::BytesMut;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct BulkString(pub(crate) Vec<u8>);

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
    const PREFIX: &'static str = "&";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        todo!()
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
