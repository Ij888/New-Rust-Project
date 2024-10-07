/// Sets up a new asynchronous web server using the provided configuration.
///
/// # Arguments
///
/// * `port` - An integer representing the port number on which the server should listen.
///
/// # Returns
///
/// * `tokio::main` - An asynchronous function that sets up the server using the provided configuration.
///
/// # Example
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let config = ServerConfig::new(8080);
///     // Use the config for your server setup...
/// }
/// ```
#[tokio::main]
async fn main() {
    let config = ServerConfig::new(8080);
    // Use the config for your server setup...
}

mod config;
