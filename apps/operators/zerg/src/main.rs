use ntex::web::{self, HttpResponse, HttpRequest, HttpServer, App};
use k8s_openapi::api::core::v1::{Pod, PodSpec, Container, Namespace};
use kube::{Client, api::{Api, ListParams, PostParams}};
use serde::{Serialize, Deserialize};
// use kube::runtime::Reflector;
// use ntex::http::body::Body;
use ntex::web::types::{Json, Path};

mod api;
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
    let client = Client::try_default().await.expect("Failed to create client");
    // let namespace = req.match_info().get("namespace").unwrap();
    // let namespace = "aris";
    // println!("namespace: {}", namespace);
    // HttpResponse::Ok().finish()
    let ns = namespace.into_inner();
    create_pod(client, ns, pod_spec.into_inner()).await
}

async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
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
    client: Client
}

impl AppState {
    fn new( client: Client) -> Self {
        Self {
            client
        }
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let client = Client::try_default().await.expect("Failed to create client");
    let state = AppState::new(client);
    HttpServer::new(move || {
        // let client = client.clone();
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            // .service(web::resource("/namespace").route(web::get().to(move || list_namespaces(client.clone()))))
            .service(web::resource("/create-pod/{namespace}").route(web::post().to(create_pod_handler)))
            .configure(routes)
            .state(state.clone())
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
