use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Write;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

use interprocess::os::windows::named_pipe::*;
use interprocess::ReliableReadMsg;

use crate::app::{ChannelCommand, DaemonProxy, ServerReceiver};

lazy_static! {
  static ref POOL: ThreadPool = ThreadPool::new(30);
}

pub fn start_server(rx: ServerReceiver, proxy: DaemonProxy) {
  thread::spawn(move || {
    let stream: PipeListener<DuplexMsgPipeStream> = PipeListenerOptions::default()
      .nonblocking(true)
      .name({
        let data: OsString = String::from("ahq_soft_framework_v_0").into();
        data
      })
      .mode(PipeMode::Messages)
      .create()
      .unwrap();

    let mut messages: HashMap<u32, DuplexMsgPipeStream> = HashMap::new();

    loop {
      if let Ok(client) = stream.accept() {
        let client_id = client.client_process_id().unwrap();

        messages.insert(client_id, client);
      }

      for msg in rx.try_iter() {
        if let Some(x) = messages.get_mut(&msg.ref_id) {
          let _ = x.write_all(&msg.data.convert());
          if let Err(_) = x.flush() {
            messages.remove(&msg.ref_id);
          }
        }
      }

      for (_, client) in &mut messages {
        let mut data = [0u8; 512];

        if let Ok(Ok(_)) = client.try_read_msg(&mut data) {}
      }
      thread::sleep(Duration::from_millis(5));
    }
  });
}
