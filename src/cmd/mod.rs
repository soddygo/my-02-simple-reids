mod map;

use enum_dispatch::enum_dispatch;
use thiserror::Error;
use crate::{Backend, RespFrame};



#[derive(Debug,Error)]
pub enum CommandError{
    
}

#[enum_dispatch]
pub trait CommandExecutor{

    fn execute(self,backend:Backend)->RespFrame;
}

#[enum_dispatch(CommandExecutor)]
#[derive(Debug)]
pub enum Command{
    Get(Get),
    Set(Set),
    Unrecognized(Unrecognized),

}

#[derive(Debug)]
pub struct Get{
    key: String,
}

#[derive(Debug)]
pub struct Set{
    key: String,
    value: RespFrame,
}


#[derive(Debug)]
pub struct Unrecognized{
}


impl TryFrom<RespFrame> for Command{
    type Error = CommandError;

    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        todo!()
    }
}