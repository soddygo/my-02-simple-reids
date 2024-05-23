use crate::resp::extract_fixed_data;
use crate::{RespDecode, RespEncode, RespError};
use bytes::BytesMut;

impl RespDecode for bool {
    const PREFIX: &'static str = "#";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        match extract_fixed_data(buf, "#t\r\n", "Bool") {
            Ok(_) => Ok(true),
            Err(RespError::NotComplete) => Err(RespError::NotComplete),
            Err(R_) => match extract_fixed_data(buf, "#f\r\n", "Bool") {
                Ok(_) => Ok(false),
                Err(e) => Err(e),
            },
        }
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        Ok(4)
    }
}

impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { "t" } else { "f" }).into_bytes()
    }
}
