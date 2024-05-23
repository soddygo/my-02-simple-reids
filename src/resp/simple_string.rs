use bytes::BytesMut;

use crate::resp::{extract_simple_frame_data, CRLF_LENGTH};
use crate::{RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleString(pub(crate) String);

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}+\r\n", self.0).into()
    }
}

impl RespDecode for SimpleString {
    const PREFIX: &'static str = "+";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end: usize = extract_simple_frame_data(buf, Self::PREFIX)?;

        let data = buf.split_to(end + CRLF_LENGTH);

        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(SimpleString(s.into()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end: usize = extract_simple_frame_data(buf, Self::PREFIX)?;

        Ok(end + CRLF_LENGTH)
    }
}

#[cfg(test)]
mod test {
    use crate::RespFrame;
    use anyhow::Result;
    use bytes::BufMut;

    use super::*;

    #[test]
    fn test_simple_string_encode() -> Result<()> {
        let frame: RespFrame = SimpleString::new("OK".to_string()).into();

        assert_eq!(frame.encode(), b"+OK+\r\n");

        Ok(())
    }

    #[test]
    fn test_simple_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"+OK\r\n");

        let frame = SimpleString::decode(&mut buf)?;

        assert_eq!(frame, SimpleString::new("OK".to_string()));

        buf.extend_from_slice(b"+hello\r");

        let ret = SimpleString::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.put_u8(b'\n');

        let frame = SimpleString::decode(&mut buf)?;
        assert_eq!(frame, SimpleString::new("hello".to_string()));

        Ok(())
    }
}
