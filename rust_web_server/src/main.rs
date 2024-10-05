/*fn main() {
    println!("Hello, world!");
}

*/
// In the file: src/main.rs

mod config;

use config::ServerConfig;

#[tokio::main]
async fn main() {
    let config = ServerConfig::new(8080);
    // Use the config for your server setup...
}
