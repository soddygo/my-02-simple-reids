use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use crate::resp::simple_string::SimpleString;
use crate::{RespDecode, RespError};

#[enum_dispatch(RespEncode)]
#[derive(Debug,Clone)]
pub enum RespFrame{

    SimpleString(SimpleString)
}


