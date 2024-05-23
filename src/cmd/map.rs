use crate::cmd::RESP_OK;
use crate::{
    Backend, CommandError, CommandExecutor, Get, RespArray, RespFrame, RespNull, Set, Unrecognized,
};

impl CommandExecutor for Get {
    fn execute(self, backend: &Backend) -> RespFrame {
        match backend.get(&self.key) {
            Some(v) => v,
            None => RespFrame::Null(RespNull),
        }
    }
}

impl CommandExecutor for Set {
    fn execute(self, backend: &Backend) -> RespFrame {
        match backend.get(&self.key) {
            None => RespFrame::Null(RespNull),
            Some(value) => value,
        }
    }
}

impl CommandExecutor for Unrecognized {
    fn execute(self, backend: &Backend) -> RespFrame {
        RESP_OK.clone()
    }
}

impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<RespArray> for Set {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        todo!()
    }
}
