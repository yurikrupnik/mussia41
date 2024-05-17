use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

fn add1(n: &i32) -> i32 {
    n + 1
}
fn adder(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

fn asd() -> Vec<i32> {
    let v1 = vec![1, 3, 5];
    let x = v1.iter().map(add1);
    let y = x.collect();
    let s = adder(1);
    let _ss = s(10);
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

use general::socket_addrs::get_web_url;

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
