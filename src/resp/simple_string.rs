use bytes::BytesMut;
use crate::{RespDecode, RespEncode, RespError, RespFrame};
use crate::resp::{CRLF_LENGTH, extract_simple_from_data};

#[derive(Debug, Clone)]
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
        let end: usize = extract_simple_from_data(buf, Self::PREFIX)?;

        let data = buf.split_to(end + CRLF_LENGTH);

        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(SimpleString(s.into()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end: usize = extract_simple_from_data(buf, Self::PREFIX)?;

        Ok(end + CRLF_LENGTH)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use tokio_util::codec::Framed;

    #[test]
    fn test_simple_string_encode() -> Result<()> {
        let frame: RespFrame = SimpleString::new("OK".to_string()).into();

        assert_eq!(frame.encode(), b"+OK+\r\n");

        Ok(())
    }
}

