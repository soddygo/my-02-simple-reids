use crate::cmd::RESP_OK;
use crate::{
    Backend, BulkString, CommandError, CommandExecutor, HGet, HGetAll, HSet, RespArray, RespFrame,
    RespNull,
};

impl CommandExecutor for HGet {
    fn execute(self, backend: &Backend) -> RespFrame {
        match backend.get(&self.key) {
            None => RespFrame::Null(RespNull),
            Some(value) => value,
        }
    }
}

impl CommandExecutor for HGetAll {
    fn execute(self, backend: &Backend) -> RespFrame {
        let hmap = backend.hmap.get(&self.key);

        match hmap {
            None => RespArray::new([]).into(),
            Some(dashmap) => {
                let mut data = Vec::with_capacity(dashmap.len());
                for v in dashmap.iter() {
                    let key = v.key().to_owned();
                    data.push((key, v.value().clone()));
                }
                if self.sort {
                    data.sort_by(|a, b| a.0.cmp(&b.0));
                }
                let ret = data
                    .into_iter()
                    .flat_map(|(k, v)| vec![BulkString::from(k).into(), v])
                    .collect::<Vec<RespFrame>>();

                RespArray::new(ret).into()
            }
        }
    }
}

impl CommandExecutor for HSet {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.hset(self.key, self.field, self.value);
        RESP_OK.clone()
    }
}

impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<RespArray> for HSet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<RespArray> for HGetAll {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        todo!()
    }
}
