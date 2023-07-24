mod models;
mod shortener;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use shortener::UrlShortener;

#[derive(serde::Deserialize)]
struct ShortenRequest {
    long_url: String,
}

#[derive(serde::Serialize)]
struct ShortenResponse {
    short_url: String,
}

#[get("/shorten")]
async fn hello(data: web::Data<UrlShortener>, path: web::Query<ShortenRequest>) -> impl Responder {
    let shorturl = &path.long_url;

    println!("{}", shorturl);
    let short_url = data.shorten_url(&path.long_url);
    HttpResponse::Ok().json(ShortenResponse {
        short_url: short_url,
    })
}

#[get("/redirect/{short_url}")]
async fn redirect(data: web::Data<UrlShortener>, path: web::Path<String>) -> impl Responder {
    let short_url = path.into_inner();
    if let Some(long_url) = data.redirect_url(&short_url) {
        HttpResponse::TemporaryRedirect()
            .header("Location", long_url)
            .finish()
    } else {
        HttpResponse::NotFound().body("Not found!")
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url_shortener = web::Data::new(UrlShortener::new());
    HttpServer::new(move || {
        App::new()
            .app_data(url_shortener.clone())
            .service(hello)
            .service(redirect)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
