use actix_web::{get, App, HttpServer, Responder};
use color_eyre::Report;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;

#[get("/")]
async fn index() -> impl Responder {
    "Hello"
}

#[actix_web::main]
async fn main() -> Result<(), Report> {
    setup()?;
    run().await?;
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}

async fn run() -> Result<(), std::io::Error> {
    let server =
        HttpServer::new(|| App::new().wrap(TracingLogger::default()).service(index));
    let address = "0.0.0.0:8080";
    info!("Starting server on {}", address);
    server.bind(address)?
        .run()
        .await
}
