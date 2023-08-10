use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use config::*;
// use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

fn load_config() -> Config {
    let mut settings = Config::default();
    settings.merge(Environment::new().prefix("APP")).unwrap();
    settings.merge(File::with_name("config")).unwrap();
    settings
}

// #[derive(Debug, Deserialize)]
// struct AppConfig {
//     database: DatabaseConfig,
// }

// #[derive(Debug, Deserialize)]
// struct DatabaseConfig {
//     url: String,
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let shared_config = Arc::new(Mutex::new(config.clone()));

    let shared_config_clone = shared_config.clone();

    tokio::spawn(async move {
        loop {
            time::sleep(Duration::from_secs(5)).await;

            let new_config = load_config();
            let mut locked_config = shared_config_clone.lock().unwrap();
            *locked_config = new_config;
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(shared_config.clone()))
            .service(web::resource("/").to(handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn handler(data: Data<Arc<Mutex<Config>>>) -> impl Responder {
    let config = data.lock().unwrap();
    let database_url: String = config.get_str("database.url").unwrap();

    HttpResponse::Ok().body(format!("Database URL: {}", database_url))
}
