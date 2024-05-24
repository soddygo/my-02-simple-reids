use crate::cmd::{extract_args, validate_command};
use crate::{Backend, BulkString, CommandError, CommandExecutor, Echo, RespArray, RespFrame};

impl CommandExecutor for Echo {
    fn execute(self, _backend: &Backend) -> RespFrame {
        let value = self.value;

        RespFrame::BulkString(BulkString::new(value))
    }
}

impl TryFrom<RespArray> for Echo {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["echo"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();

        match args.next() {
            Some(RespFrame::BulkString(value)) => Ok(Echo {
                value: String::from_utf8(value.0)?,
            }),
            _ => Err(CommandError::InvalidArguments("Invalid key".to_string())),
        }
    }
}
