use actix_cors::Cors;
use actix_web::{get, http, post, web, App, Error, HttpResponse, HttpServer};
use std::sync::{Mutex};

type TodoListType = Vec<String>;
type TodoListMutexType = Mutex<TodoListType>;

#[get("/")]
async fn index() -> String {
  format!("Welcome to the Rust Todo Server!")
}

#[get("/todos")]
async fn get_todos(todos_mutex: web::Data<TodoListMutexType>) -> HttpResponse {
  HttpResponse::Ok()
    .content_type("application/json")
    .json(&**todos_mutex)
}

#[post("/todo/{todo}")]
async fn post_todo(todos_mutex: web::Data<TodoListMutexType>, todo: web::Path<String>) -> Result<HttpResponse, Error> {
  let mut todos_here = todos_mutex.lock().unwrap();
  if todos_here.len() < 5 {
    todos_here.push(todo.to_string());
    return HttpResponse::Ok().await
  }
  else {
    return HttpResponse::Conflict().await
  }
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

    let mut todos:TodoListType = Vec::new();
    todos.push("testingtesting123".to_string());
    let todos_mutex:TodoListMutexType = Mutex::new(todos);

    App::new()
      .wrap(cors)
      .data(todos_mutex)
      .service(index)
      .service(get_todos)
      .service(post_todo)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}