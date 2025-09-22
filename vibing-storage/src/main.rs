use axum::{
    Router,
    routing::{get, post},
    serve,
};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::cors::{Any, CorsLayer};

use vibing_storage::{
    app::api::{
        delete::delete_track,
        get::{get_filtered_page, get_root, handle_download_request, handle_stream_request},
        patch::update_track,
        post::handle_upload_request,
    },
    config::Configuration,
    database::core::pool::VibingPool,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    #[cfg(feature = "init_db")]
    let pool = VibingPool::init().await;
    #[cfg(not(feature = "init_db"))]
    let pool = VibingPool::get().await;

    let pool = Arc::new(RwLock::new(pool));

    #[cfg(feature = "get_resource")]
    {
        use database::entities::track::TrackFull;
        let metadata_vec =
            vibing_storage::app::fetch::fetch_resource_from(&Configuration::get().resource_dir)
                .expect("cannot get resource");
        for metadata in metadata_vec {
            TrackFull::create_from(metadata, pool.clone())
                .await
                .expect("cannot store resource");
        }
    }

    let address = format!("127.0.0.1:{}", Configuration::get().port);
    let listener = TcpListener::bind(address)
        .await
        .expect("cannot bind address");

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse().unwrap(),
            "https://glacial.site".parse().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(get_root))
        .route(
            "/tracks",
            get(get_filtered_page)
                .patch(update_track)
                .delete(delete_track),
        )
        .route("/tracks/download", get(handle_download_request))
        .route("/tracks/upload", post(handle_upload_request))
        .route("/tracks/stream", get(handle_stream_request))
        .with_state(pool)
        .layer(cors);

    serve(listener, app.into_make_service())
        .await
        .expect("cannot serving app");
}
