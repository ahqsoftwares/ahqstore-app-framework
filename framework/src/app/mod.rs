use serde::Serialize;
use std::sync::mpsc::{channel, Receiver, Sender};

use wry::application::event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy};

mod structs;
pub use structs::*;

pub type ServerReceiver = Receiver<Command<CommandEnum>>;
pub type DaemonProxy = EventLoopProxy<CommandEnum>;

pub struct Application<T: ChannelCommand + Serialize + 'static> {
  pub tx: Sender<Command<T>>,
  pub rx: Receiver<Command<T>>,
  pub application: EventLoop<T>,
  pub proxy: EventLoopProxy<T>,
}

pub fn start_app() -> Application<CommandEnum> {
  let application: EventLoop<CommandEnum> = EventLoopBuilder::with_user_event().build();

  let proxy: EventLoopProxy<CommandEnum> = application.create_proxy();
  let (tx, rx) = channel::<Command<CommandEnum>>();

  Application {
    application,
    proxy,
    rx,
    tx,
  }
}
