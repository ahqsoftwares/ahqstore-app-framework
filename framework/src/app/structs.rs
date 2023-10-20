use serde::{Deserialize, Serialize};
use serde_json::to_string;

pub trait ChannelCommand {
  fn convert(&self) -> Vec<u8>;
}

pub struct Command<T: ChannelCommand> {
  pub ref_id: u32,
  pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandEnum {}

impl ChannelCommand for CommandEnum {
  fn convert(&self) -> Vec<u8> {
    to_string(self).unwrap_or("ERR".into()).as_bytes().into()
  }
}
