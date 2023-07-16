pub struct App {
    _port: String,
    _password: String
}

impl App {
    pub fn start() -> Self {
        Self {
            _port: String::from("8080"),
            _password: String::from("password")
        }
    }
}