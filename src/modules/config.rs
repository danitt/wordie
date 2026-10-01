use std::env;

pub struct AnkiConfig {
    pub username: String,
    pub password: String,
    pub chrome_path: Option<String>,
}

pub struct DiscordConfig {
    pub token: String,
    pub channel_id: String,
}

fn required_var(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("Missing required environment variable {}", name))
}

pub fn load_env_file() {
    dotenvy::dotenv().ok();
}

pub fn anki_config() -> Result<AnkiConfig, String> {
    Ok(AnkiConfig {
        username: required_var("ANKI_USER")?,
        password: required_var("ANKI_PASS")?,
        chrome_path: env::var("CHROME_PATH").ok(),
    })
}

pub fn discord_config() -> Result<DiscordConfig, String> {
    Ok(DiscordConfig {
        token: required_var("DISCORD_TOKEN")?,
        channel_id: required_var("DISCORD_CHANNEL_ID")?,
    })
}

pub fn history_path() -> String {
    env::var("WOD_HISTORY_PATH").unwrap_or_else(|_| crate::r#const::WOD_HISTORY_PATH.to_string())
}
