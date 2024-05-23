use bytes::BytesMut;

use crate::resp::{extract_simple_frame_data, CRLF_LENGTH};
use crate::{RespDecode, RespEncode, RespError};

impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "-" } else { "+" };
        format!("{}{}\r\n", sign, self.abs()).into_bytes()
    }
}

impl RespDecode for i64 {
    const PREFIX: &'static str = ":";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let data = buf.split_to(end + CRLF_LENGTH);

        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);

        Ok(s.parse::<i64>()?)
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;

        Ok(end + CRLF_LENGTH)
    }
}
