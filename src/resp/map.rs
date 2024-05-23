use crate::resp::{calc_total_length, parse_length, BUF_CAP, CRLF_LENGTH};
use crate::{RespDecode, RespEncode, RespError, RespFrame, SimpleString};
use bytes::{Buf, BytesMut};
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct RespMap(pub(crate) BTreeMap<String, RespFrame>);

//%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>

impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);

        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());

        for (key, value) in self.0 {
            buf.extend_from_slice(&SimpleString::new(key).encode());
            buf.extend_from_slice(&value.encode())
        }

        buf
    }
}

impl RespDecode for RespMap {
    const PREFIX: &'static str = "%";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }
        buf.advance(end + CRLF_LENGTH);

        let mut frames = RespMap::new();

        for _ in 0..len {
            let key = SimpleString::decode(buf)?;
            let value = RespFrame::decode(buf)?;
            frames.insert(key.0, value);
        }

        Ok(frames)
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RespMap {
    pub fn new() -> Self {
        RespMap(BTreeMap::new())
    }
}
