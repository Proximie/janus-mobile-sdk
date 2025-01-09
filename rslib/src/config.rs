#[derive(uniffi::Record)]
pub struct Config {
    pub url: String,
    pub capacity: u16,
    pub apisecret: Option<String>,
    pub server_root: Option<String>,
}
