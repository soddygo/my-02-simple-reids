use bytes::{Buf, BytesMut};
use enum_dispatch::enum_dispatch;
use thiserror::Error;

mod frame;
mod simple_string;
mod null;

pub use frame::*;

pub use self::{
    simple_string::*,
    null::RespNull,
};

const BUF_CAP: usize = 4096;
const CRLF: &[u8] = b"\r\n";
const CRLF_LENGTH: usize = CRLF.len();

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    const PREFIX: &'static str;

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;

    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

#[derive(Debug, Error, PartialEq)]
pub enum RespError {
    #[error("Frame not enough data")]
    NotComplete,

    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
}


fn extract_fixed_data(buf: &mut BytesMut,
                      expect: &str,
                      expect_type: &str,
) -> Result<(), RespError> {
    if buf.len() < expect.len() {
        return Err(RespError::NotComplete);
    }
    if !buf.starts_with(expect.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "expert frame type:{},but get  {:?}",
            expect_type, buf
        )));
    }

    buf.advance(expect.len());

    Ok(())
}


fn extract_simple_from_data(buf: &[u8], prefix: &str) -> Result<usize, RespError> {
    if buf.len() < 3 {
        return Err(RespError::NotComplete);
    }
    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "expert frame type:{},but get  {:?}",
            prefix, buf
        )));
    }

    let end = find_crlf(buf, 1).ok_or(RespError::NotComplete)?;

    Ok(end)
}

fn find_crlf(buf: &[u8], nth: usize) -> Option<usize> {
    let mut count = 0;

    for x in 1..buf.len() - 1 {
        if buf[x] == b'\r' && buf[x + 1] == b'\n' {
            count += 1;
            if count == nth {
                return Some(x);
            }
        }
    }

    None
}
