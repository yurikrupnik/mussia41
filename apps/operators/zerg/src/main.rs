use k8s_openapi::api::core::v1::{Container, Namespace, Pod, PodSpec};
use kube::{
    api::{Api, ListParams, PostParams},
    Client,
};
use ntex::web::{self, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use services::{
    // mongo::connector::connect as mongo_connect, redis::connector::connect as redis_connect,
    swagger::ntex::ntex_config,
};
// use kube::runtime::Reflector;
// use ntex::http::body::Body;
use general::socket_addrs::get_web_url;
use ntex::web::types::{Json, Path};

mod api;
mod swagger;
use swagger::ApiDoc;
// mod controller;
// mod crdgen;
// mod fixtures;

use api::routes;

#[derive(Deserialize)]
struct PodSpecInput {
    name: String,
    image: String,
}

#[derive(Deserialize)]
struct NamespacePath {
    namespace: String,
}

async fn create_pod_handler(namespace: Path<String>, pod_spec: Json<PodSpecInput>) -> HttpResponse {
    let client = Client::try_default()
        .await
        .expect("Failed to create client");
    // let namespace = req.match_info().get("namespace").unwrap();
    // let namespace = "aris";
    // println!("namespace: {}", namespace);
    // HttpResponse::Ok().finish()
    let ns = namespace.into_inner();
    create_pod(client, ns, pod_spec.into_inner()).await
}

async fn create_pod(client: Client, namespace: String, pod_spec: PodSpecInput) -> HttpResponse {
    let pods: Api<Pod> = Api::namespaced(client, &namespace);

    let pod = Pod {
        metadata: kube::api::ObjectMeta {
            name: Some(pod_spec.name.clone()),
            ..Default::default()
        },
        spec: Some(PodSpec {
            containers: vec![Container {
                name: pod_spec.name.clone(),
                image: Some(pod_spec.image.clone()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    // match pods.delete(&PostParams::default(), &pod) {  }
    match pods.create(&PostParams::default(), &pod).await {
        Ok(o) => HttpResponse::Ok().json(&o),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating pod: {:?}", e)),
    }
}

#[derive(Clone)]
struct AppState {
    client: Client,
}

impl AppState {
    fn new(client: Client) -> Self {
        Self { client }
    }
}

async fn default() -> HttpResponse {
    HttpResponse::NotFound().finish()
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let client = Client::try_default()
        .await
        .expect("Failed to create client");
    let state = AppState::new(client);
    HttpServer::new(move || {
        // let json_config = web::types::JsonConfig::default().limit(11);
        App::new()
            .configure(ntex_config::<ApiDoc>)
            .wrap(web::middleware::Logger::default())
            // .state(json_config)
            // .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/create-pod/{namespace}").route(web::post().to(create_pod_handler)),
            )
            .configure(routes)
            .state(state.clone())
            .default_service(web::route().to(default))
    })
    .bind(get_web_url(false))?
    .run()
    .await
}
