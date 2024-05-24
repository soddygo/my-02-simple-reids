use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use log::{info, warn};

use crate::resp::array::RespArray;
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
        info!("rev buf: {:?}", buf);

        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(&b'+') => {
                let frame = SimpleString::decode(buf)?;

                Ok(frame.into())
            }
            Some(b'-') => {
                let frame = SimpleError::decode(buf)?;
                Ok(frame.into())
            }
            Some(b':') => {
                let frame = i64::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'$') => {
                // try null bulk string first
                match BulkString::decode(buf) {
                    Ok(frame) => Ok(frame.into()),
                    Err(RespError::NotComplete) => Err(RespError::NotComplete),
                    Err(_) => Err(RespError::InvalidFrameType(format!(
                        "invalid frame type: {:?}",
                        String::from_utf8_lossy(buf)
                    ))),
                }
            }
            Some(b'*') => {
                // try null array first
                match RespArray::decode(buf) {
                    Ok(frame) => Ok(frame.into()),
                    Err(RespError::NotComplete) => Err(RespError::NotComplete),
                    Err(_) => Err(RespError::InvalidFrameType(format!(
                        "invalid frame type: {:?}",
                        String::from_utf8_lossy(buf)
                    ))),
                }
            }
            Some(b'_') => {
                let frame = RespNull::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'#') => {
                let frame = bool::decode(buf)?;
                Ok(frame.into())
            }
            Some(b',') => {
                let frame = f64::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'%') => {
                let frame = RespMap::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'~') => {
                let frame = RespSet::decode(buf)?;
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
            Some(b'*') => RespArray::expect_length(buf),
            Some(b'~') => RespSet::expect_length(buf),
            Some(b'%') => RespMap::expect_length(buf),
            Some(b'$') => BulkString::expect_length(buf),
            Some(b':') => i64::expect_length(buf),
            Some(b'+') => SimpleString::expect_length(buf),
            Some(b'-') => SimpleError::expect_length(buf),
            Some(b'#') => bool::expect_length(buf),
            Some(b',') => f64::expect_length(buf),
            Some(b'_') => RespNull::expect_length(buf),

            _ => {
                warn!("expect_length: may be not complete: {:?}", buf);

                Err(RespError::NotComplete)
            }
        }
    }
}
