use std::ops::{Deref, DerefMut};

use bytes::{Buf, BytesMut};

use crate::resp::{calc_total_length, parse_length, BUF_CAP, CRLF_LENGTH};
use crate::{RespDecode, RespEncode, RespError, RespFrame};

//~<number-of-elements>\r\n<element-1>...<element-n>

#[derive(Debug, Clone, PartialEq)]
pub struct RespSet(pub(crate) Vec<RespFrame>);

impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("~{}", self.0.len()).into_bytes());

        for frame in self.0 {
            buf.extend_from_slice(&frame.encode())
        }
        buf
    }
}

impl RespDecode for RespSet {
    const PREFIX: &'static str = "~";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if total_len > buf.len() {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LENGTH);
        let mut set = RespSet::new();
        for _ in 0..len {
            let frame = RespFrame::decode(buf)?;
            set.push(frame);
        }

        Ok(set)
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;

        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RespSet {
    fn new() -> Self {
        Self(Vec::new())
    }
}
