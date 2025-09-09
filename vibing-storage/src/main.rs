use axum::{routing::get, serve, Router};
use tokio::net::TcpListener;
use vibing_storage::{app::apis::get::get_root, config::Configuration, database::{core::pool::VibingPool}};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

#[cfg(feature = "init_db")]
    let pool = VibingPool::init().await;
#[cfg(not(feature = "init_db"))]
    let pool = VibingPool::get().await;

#[cfg(feature = "get_sample")]
{
    vibing_storage::app::fetch::SampleRoot::fetch()
        .sync(&pool).await
        .expect("cannot sync sample");
}

    let address = format!("127.0.0.1:{}", Configuration::get().port);
    let listener = TcpListener::bind(address).await
        .expect("cannot bind address");
    let app = Router::new()
        .route("/", get(get_root));

    serve(listener, app.into_make_service()).await
        .expect("cannot serving app");
}
