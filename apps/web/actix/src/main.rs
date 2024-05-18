use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use general::socket_addrs::get_web_url;

fn adder(a: i32) -> impl Fn(&i32) -> i32 {
    move |b| a + b
}

fn asd() -> Vec<i32> {
    let v1 = vec![1, 3, 5];
    let s = adder(1);
    let x = v1.iter().map(adder(12));
    let y = x.collect();
    let _ss = s(&10);
    y
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let _ = asd();
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Connected to mongo");
    // s.
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(get_web_url(false))?
    .run()
    .await
}
