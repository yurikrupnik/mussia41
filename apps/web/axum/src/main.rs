// TODO play with the code:
// https://github.com/tokio-rs/axum/blob/main/examples/prometheus-metrics/src/main.rs

use axum::Router;
// use c::PrometheusMetricLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use api::routes;
use general::socket_addrs::get_web_url_v1;
use services::{
  mongo::connector::connect as mongo_connect, redis::connector::connect as redis_connect,
};
use shared::app_state::AppState;
use swagger::ApiDoc;
use webserver_axum::logger::set_logger;

mod api;
mod swagger;
mod log;
mod web;

pub fn fd<T: OpenApi>() -> Router {
  Router::new()
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", T::openapi()))
    .merge(Redoc::with_url("/redoc", T::openapi()))
    .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    .merge(Scalar::with_url("/scalar", T::openapi()))
}

#[tokio::main]
async fn main() {
  // Set up logging
  set_logger();

  // let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
  // let metric_handler = || async move { metric_handle.render() };

  let db = mongo_connect("aris").await;
  let redis_pool = redis_connect().await;
  let state = AppState::new(db, redis_pool);
  let app = Router::new()
    // .merge(fd::<ApiDoc>())
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
    .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
    .merge(routes())
    // .route(
    //   "/metrics",
    //   axum::routing::get(metric_handler),
    // )
    // .route(
    //   "/health",
    // axum::routing::get(|_| async move { metric_handle.render() }),
    // )
    // .layer(prometheus_layer)
    .layer(TraceLayer::new_for_http())
    .with_state(state);

  // run our app with hyper, listening globally on port
  let listener_value = get_web_url_v1();
  let listener = tokio::net::TcpListener::bind(listener_value)
    .await
    .expect("could not bind to address");
  tracing::debug!(
        "listening in {}",
        listener.local_addr().expect("could not get local address")
    );
  tracing::error!(
        "listening in {}",
        listener.local_addr().expect("could not get local address")
    );
  tracing::info!(
        "listening in {}",
        listener.local_addr().expect("could not get local address")
    );
  tracing::trace!(
        "listening in {}",
        listener.local_addr().expect("could not get local address")
    );
  axum::serve(listener, app).await.unwrap();
}
