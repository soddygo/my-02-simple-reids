use crate::{Backend, CommandExecutor, Get, RespFrame, RespNull, Set, Unrecognized};

impl CommandExecutor for Get{
    fn execute(self, backend: Backend) -> RespFrame {
        match backend.get(&self.key) {
            Some(v) => v,
            None => RespFrame::Null(RespNull),
        }
    }
}


impl CommandExecutor for Set{
    fn execute(self, backend: Backend) -> RespFrame {
        todo!()
    }
}

impl CommandExecutor for Unrecognized{
    fn execute(self, backend: Backend) -> RespFrame {
        todo!()
    }
}