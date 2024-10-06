pub const UPLOADS_DIRECTORY: &str = "uploads"; // The directory where we store file uploads
pub const DEFAULT_PORT: &str = "3000"; // This is stored as a string to match environment vars
pub const DEFAULT_HOST: &str = "0.0.0.0";

pub struct Config {
    pub port: String,
    pub host: String,
    pub uploads_dir: String,
}

impl Config {
    pub fn new() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()
            .expect("PORT must be a number");

        let host = std::env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());

        let uploads_dir =
            std::env::var("UPLOADS_DIR").unwrap_or_else(|_| UPLOADS_DIRECTORY.to_string());

        Config {
            port,
            host,
            uploads_dir,
        }
    }
    /// Formats the host and port into an address for a TCPListener to bind to
    pub fn get_address(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }
}
