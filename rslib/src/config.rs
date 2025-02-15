#[derive(uniffi::Record)]
pub struct Config {
    pub url: String,
    pub capacity: u16,
    pub apisecret: Option<String>,
    #[uniffi(default = "janus")]
    pub server_root: String,
}
