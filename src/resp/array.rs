use std::ops::Deref;

use bytes::{Buf, BytesMut};

use crate::resp::{calc_total_length, extract_fixed_data, parse_length, BUF_CAP, CRLF_LENGTH};
use crate::{RespDecode, RespEncode, RespError, RespFrame};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RespArray(pub(crate) Vec<RespFrame>);

#[derive(Debug, Clone, PartialEq)]
pub struct RespNullArray;

impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);

        buf.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());

        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }

        buf
    }
}
// - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
// - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
// FIXME: need to handle incomplete
impl RespDecode for RespArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf.as_ref(), Self::PREFIX)?;
        let total = calc_total_length(buf.as_ref(), end, len, Self::PREFIX)?;

        if total > buf.len() {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LENGTH);

        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            let frame = RespFrame::decode(buf)?;
            frames.push(frame);
        }
        Ok(RespArray(frames))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

impl RespDecode for RespNullArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "*-1\r\n", "NullArray")?;
        Ok(RespNullArray)
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(5)
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl RespArray {
    pub(crate) fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(s.into())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use log::info;

    #[test]
    fn test_array_null_length() -> Result<()> {
        let data = b"*-1\r\n";

        info!("data.len: {:?}", data.len());
        assert_eq!(data.len(), 5);

        Ok(())
    }
}
