mod app;

fn main() {
  let application = app::start_app();

  application.application.run(|_, _, _| {});
}
