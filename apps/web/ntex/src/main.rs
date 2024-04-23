use ntex::web;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
#[web::get("/users/{user_id}/{friend}")] // <- define path parameters
async fn friend1(info: web::types::Path<Info>) -> Result<String, web::Error> {
    // let sd = info.into();
    // info.into_inner().friend
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}
#[web::get("/")]
async fn welcome(_req: web::HttpRequest) -> impl web::Responder {
    "Welcome!"
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[web::get("/users/{user_id}/{friend}")] // <- define path parameters
async fn friend(path: web::types::Path<(u32, String)>) -> Result<String, web::Error> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    web::HttpServer::new(|| web::App::new()
        .service(welcome)
        .service(friend1)
        .service(friend)
    )
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}