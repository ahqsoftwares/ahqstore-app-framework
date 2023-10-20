use wry::application::event::Event;

mod app;
mod server;

fn main() {
  let application = app::start_app();

  server::start_server(application.rx, application.proxy);

  application.application.run(|event, _, _| match event {
    Event::UserEvent(_) => {}
    Event::WindowEvent { .. } => {}
    _ => {}
  });
}
