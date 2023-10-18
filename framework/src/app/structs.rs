use serde::{Deserialize, Serialize};

pub trait ChannelCommand {}

pub struct Command<T: ChannelCommand> {
  pub ref_id: u64,
  pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandEnum {}

impl ChannelCommand for CommandEnum {}
