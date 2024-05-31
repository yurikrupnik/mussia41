mod api;
mod swagger;
use api::routes;
use axum::Router;
use general::socket_addrs::get_web_url_v1;
use services::{
    mongo::connector::connect as mongo_connect, redis::connector::connect as redis_connect,
};
use shared::app_state::AppState;
use swagger::ApiDoc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // Set up logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "info".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = mongo_connect("aris").await;
    let redis_pool = redis_connect().await;
    let state = AppState::new(db, redis_pool);
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .merge(routes())
        .with_state(state);

    // run our app with hyper, listening globally on port
    let listener_value = get_web_url_v1();
    let listener = tokio::net::TcpListener::bind(listener_value).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
