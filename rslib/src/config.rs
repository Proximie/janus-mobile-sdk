#[derive(uniffi::Record)]
pub struct Config {
    /// Server URL
    pub url: String,
    /// Buffer capacity
    pub capacity: u16,
    /// API secret
    #[uniffi(default = None)]
    pub apisecret: Option<String>,
    #[uniffi(default = "janus")]
    pub server_root: String,
}
