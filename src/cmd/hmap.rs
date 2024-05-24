use crate::cmd::{extract_args, validate_command, RESP_OK};
use crate::{
    Backend, BulkString, CommandError, CommandExecutor, HGet, HGetAll, HSet, RespArray, RespFrame,
    RespNull,
};

impl CommandExecutor for HGet {
    fn execute(self, backend: &Backend) -> RespFrame {
        match backend.hget(&self.key, &self.field) {
            None => RespFrame::Null(RespNull),
            Some(value) => value,
        }
    }
}

impl CommandExecutor for HGetAll {
    fn execute(self, backend: &Backend) -> RespFrame {
        let hmap = backend.hmap.get(&self.key);

        match hmap {
            None => RespArray::nill_new().into(),
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
        validate_command(&value, &["hget"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();

        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) => Ok(HGet {
                key: String::from_utf8(key.0)?,
                field: String::from_utf8(field.0)?,
            }),
            _ => Err(CommandError::InvalidArguments(
                "Invalid key or field".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for HSet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hset"], 3)?;
        let mut args = extract_args(value, 1)?.into_iter();

        match (args.next(), args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field)), Some(value)) => {
                Ok(HSet {
                    key: String::from_utf8(key.0)?,
                    field: String::from_utf8(field.0)?,
                    value,
                })
            }
            _ => Err(CommandError::InvalidArguments(
                "Invalid key, field or value".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for HGetAll {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hgetall"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(HGetAll {
                key: String::from_utf8(key.0)?,
                sort: false,
            }),
            _ => Err(CommandError::InvalidArguments("Invalid key".to_string())),
        }
    }
}
