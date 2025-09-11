use std::sync::Arc;

use axum::{routing::get, serve, Router};
use tokio::{net::TcpListener, sync::RwLock};
use vibing_storage::{app::apis::get::{download_track_by_id, get_root, get_tracks_by_filter}, config::Configuration, database::core::pool::VibingPool};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

#[cfg(feature = "init_db")]
    let pool = VibingPool::init().await;
#[cfg(not(feature = "init_db"))]
    let pool = VibingPool::get().await;

    let pool = Arc::new(RwLock::new(pool));

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
        .route("/", get(get_root))
        .route("/tracks", get(get_tracks_by_filter))
        .route("download", get(download_track_by_id))
        .with_state(pool);

    serve(listener, app.into_make_service()).await
        .expect("cannot serving app");
}
