// In the file: src/config.rs

pub struct ServerConfig {
    pub port: u16,
}

impl ServerConfig {
    pub fn new(port: u16) -> Self {
        ServerConfig { port }
    }
}