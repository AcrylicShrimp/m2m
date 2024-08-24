#![forbid(unsafe_code)]

mod services;

use figment::{providers::Env, Figment};
use rocket::{build, main, routes};

#[main]
async fn main() -> Result<(), anyhow::Error> {
    let config = config();
    let db_service = services::create_db_service(&config.focus("postgres")).await?;

    build()
        .manage(db_service)
        .mount("/", routes![::routes::index])
        .launch()
        .await?;

    Ok(())
}

fn config() -> Figment {
    Figment::new().merge(Env::prefixed("m2m_").split("_"))
}
