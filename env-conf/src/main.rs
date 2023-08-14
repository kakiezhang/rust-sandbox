use serde::{Deserialize, Serialize};

// APP_CONF_PATH=config.toml cargo run
fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::Environment::new().prefix("APP"))
        .unwrap();
    // println!("{:?}", settings);
    let conf_path = settings.get_str("conf_path").unwrap();
    // println!("{:?}", conf_path);
    settings.merge(config::File::with_name(&conf_path)).unwrap();
    println!("settings: {:?}", settings);

    let botconf: BotConfig = settings.clone().try_into().unwrap();
    println!("botconf: {:?}", botconf);
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BotConfig {
    pub conf_path: String,
    pub sys: Sys,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Sys {
    name: String,
}
