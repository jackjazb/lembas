use axum::{routing::get, Router};
use std::{env, error::Error, process::exit};
use tracing::{event, Level};

use crate::api::db::Db;

mod api;
mod router;

const PORT: usize = 3000;

#[tokio::main]
async fn main() {
    #[cfg(feature = "dev")]
    event!(Level::INFO, "starting server in development mode");

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    event!(
        Level::INFO,
        "database: {}",
        env::var("DATABASE_URL").unwrap_or("".into())
    );
    event!(
        Level::INFO,
        "idp: {}",
        env::var("IDP_URL").unwrap_or("".into())
    );

    let server = start_server().await;
    if server.is_err() {
        event!(Level::ERROR, "failed to initialise: {:?}", server);

        let router = Router::new().route("/", get(bad_config_warning));
        let server = axum::Server::bind(&format!("0.0.0.0:{PORT}").parse().unwrap())
            .serve(router.into_make_service());
        if let Err(err) = server.await {
            event!(Level::ERROR, "failed to start server: {:?}", err);
        }
    }
}

async fn start_server() -> Result<(), Box<dyn Error>> {
    // Create database connection and check for IDP url.
    let db = Db::new().await?;
    let idp_url = env::var("IDP_URL")?;

    // Security must be explicitly disabled.
    let idp_enabled_conf = env::var("IDP_ENABLED").unwrap_or("true".into());
    let idp_enabled: bool = idp_enabled_conf.parse()?;

    // Run database migrations
    if let Err(e) = db.migrate().await {
        println!("Failed to run SQLx migrations: {:?}", e);
        exit(0)
    }

    #[cfg(feature = "dev")]
    {
        let dataload = load_dev_data(&db).await;
        match dataload {
            Ok(_) => event!(Level::INFO, "development data loaded"),
            Err(e) => event!(Level::ERROR, "development data error: {:#?}", e),
        }
    }

    // Create routes. All requests are routed through the authorise() middleware
    let router = router::make_router(db, idp_url, idp_enabled).await?;

    // Start the backend
    let server = axum::Server::bind(&format!("0.0.0.0:{PORT}").parse().unwrap())
        .serve(router.into_make_service());
    event!(Level::INFO, "server started");

    Ok(server.await?)
}

// basic handler that responds with a static string
async fn bad_config_warning() -> &'static str {
    "Configuration error - please restart the instance with DATABASE_URL and IDP_URL set."
}

#[cfg(feature = "dev")]
async fn load_dev_data(db: &Db) -> Result<(), sqlx::Error> {
    let sql = include_str!("../db/devdata.sql");
    let queries = sql.split(';');
    for query in queries {
        sqlx::query(query).execute(&db.pool).await?;
    }
    Ok(())
}
