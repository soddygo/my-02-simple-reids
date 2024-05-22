use crate::resp::simple_string::SimpleString;
use crate::{RespDecode, RespError};
use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use crate::resp::null::RespNull;

#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Null(RespNull)
}


impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(&b'+') => {
             let frame =   SimpleString::decode(buf)?;
                
                Ok(frame.into())
            }
            _ => Err(RespError::NotComplete),
            
        }
        
      
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(&b'+') => {
                SimpleString::expect_length(buf)
            }
            _ => Err(RespError::NotComplete),
        }
    }
}
