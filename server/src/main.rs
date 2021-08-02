use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct ActixData {
  counter: i128,
}

#[get("/")]
async fn index() -> String {
  println!("testing123");
  format!("Welcome to the Rust Counter Server!")
}

#[get("/count")]
async fn get_count(actix_data: web::Data<Arc<Mutex<ActixData>>>) -> HttpResponse {
  HttpResponse::Ok().body(format!("{:?}", actix_data.lock().unwrap().counter))
}

#[post("/increment")]
async fn increment(actix_data: web::Data<Arc<Mutex<ActixData>>>) -> HttpResponse {
  actix_data.lock().unwrap().counter += 1;
  HttpResponse::Ok().body(format!("{:?}", actix_data.lock().unwrap().counter))
}

#[post("/decrement")]
async fn decrement(actix_data: web::Data<Arc<Mutex<ActixData>>>) -> HttpResponse {
  actix_data.lock().unwrap().counter -= 1;
  HttpResponse::Ok().body(format!("{:?}", actix_data.lock().unwrap().counter))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    let cors = Cors::default()
      .allowed_origin("http://localhost:3000")
      .allowed_origin("https://harryli0088.github.io")
      .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
      .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
      .allowed_header(http::header::CONTENT_TYPE)
      .max_age(3600);

    let data = Arc::new(Mutex::new(ActixData::default()));

    App::new()
      .wrap(cors)
      .data(data.clone())
      .service(index)
      .service(get_count)
      .service(increment)
      .service(decrement)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}