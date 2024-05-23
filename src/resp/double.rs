use crate::resp::{extract_simple_frame_data, CRLF_LENGTH};
use crate::{RespDecode, RespEncode, RespError};
use bytes::BytesMut;

//,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);

        let ret = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };

            format!(",{}{}\r\n", sign, self)
        };

        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

impl RespDecode for f64 {
    const PREFIX: &'static str = ",";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let data = buf.split_to(end + CRLF_LENGTH);

        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);

        Ok(s.parse::<f64>()?)
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;

        Ok(end + CRLF_LENGTH)
    }
}

// todo!("The positive infinity, negative infinity and NaN values are encoded as follows:")
/*
,inf\r\n
,-inf\r\n
,nan\r\n
 */
