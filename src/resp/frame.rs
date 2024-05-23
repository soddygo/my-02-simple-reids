use bytes::BytesMut;
use enum_dispatch::enum_dispatch;

use crate::resp::array::{RespArray, RespNullArray};
use crate::resp::bulk_string::BulkString;
use crate::resp::map::RespMap;
use crate::resp::null::RespNull;
use crate::resp::set::RespSet;
use crate::resp::simple_error::SimpleError;
use crate::resp::simple_string::SimpleString;
use crate::{RespDecode, RespError};

#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone, PartialEq)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Null(RespNull),
    Double(f64),
    Array(RespArray),
    NullArray(RespNullArray),
    Map(RespMap),
    BulkString(BulkString),
    SimpleError(SimpleError),
    Bool(bool),
    Integer(i64),
    Set(RespSet),
}

impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(&b'+') => {
                let frame = SimpleString::decode(buf)?;

                Ok(frame.into())
            }
            None => Err(RespError::NotComplete),
            _ => Err(RespError::InvalidFrameType(format!(
                "expect_length: unknown frame type: {:?}",
                buf
            ))),
        }
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => SimpleString::expect_length(buf),

            _ => Err(RespError::NotComplete),
        }
    }
}
