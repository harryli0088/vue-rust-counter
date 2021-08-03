use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer};
use std::{
  env,
  sync::{Arc, Mutex},
};

#[derive(Debug, Default)]
struct ActixData {
  counter: u8,
}

#[get("/")]
async fn index() -> String {
  format!("Welcome to the Rust Counter Server!")
}

#[get("/count")]
async fn get_count(actix_data: web::Data<Arc<Mutex<ActixData>>>) -> HttpResponse {
  HttpResponse::Ok().body(format!("{:?}", actix_data.lock().unwrap().counter))
}

#[post("/increment")]
async fn increment(actix_data: web::Data<Arc<Mutex<ActixData>>>) -> HttpResponse {
  if actix_data.lock().unwrap().counter.checked_add(1) <= Some(u8::MAX) {
    actix_data.lock().unwrap().counter += 1;
    HttpResponse::Ok().body(format!("{:?}", actix_data.lock().unwrap().counter))
  }
  else {
    HttpResponse::Conflict().body(format!("Cannot increment anymore"))
  }
}

#[post("/decrement")]
async fn decrement(actix_data: web::Data<Arc<Mutex<ActixData>>>) -> HttpResponse {
  if actix_data.lock().unwrap().counter.checked_sub(1) >= Some(u8::MIN) {
    actix_data.lock().unwrap().counter -= 1;
    HttpResponse::Ok().body(format!("{:?}", actix_data.lock().unwrap().counter))
  }
  else {
    HttpResponse::Conflict().body(format!("Cannot decrement anymore"))
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  //define the app state outside the HttpServer closure so that it persists
  //https://stackoverflow.com/questions/59276996/why-does-my-shared-actix-web-state-sometimes-reset-back-to-the-original-value
  let data = Arc::new(Mutex::new(ActixData::default()));

  // Get the port number to listen on (required for heroku deployment).
  let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
  let server_addr = format!("0.0.0.0:{}", port);

  HttpServer::new(move || {
    let cors = Cors::default()
      .allowed_origin("http://localhost:3000")
      .allowed_origin("https://harryli0088.github.io")
      .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
      .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
      .allowed_header(http::header::CONTENT_TYPE)
      .max_age(3600);

    App::new()
      .wrap(cors)
      .data(data.clone())
      .service(index)
      .service(get_count)
      .service(increment)
      .service(decrement)
  })
  .bind(server_addr)?
  .run()
  .await
}