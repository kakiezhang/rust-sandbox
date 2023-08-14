// APP_CONF_PATH=1 cargo run
fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::Environment::new().prefix("APP"))
        .unwrap();
    println!("{:?}", settings);
    let conf_path = settings.get_str("conf_path").unwrap();
    println!("{:?}", conf_path);
}
