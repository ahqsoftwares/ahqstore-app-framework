pub struct App {
    port: String,
    password: String
}

impl App {
    pub fn start() -> Self {
        Self {
            port: String::from("8080"),
            password: String::from("password")
        }
    }
}