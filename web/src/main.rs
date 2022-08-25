use std::sync::{Mutex};

use actix_web::{get, post, web, App, HttpServer, Responder, middleware, HttpResponse, http::header::ContentType};
use serde::{Serialize, Deserialize};

struct AppState {
    repository: TodoRepository,
}

struct TodoRepository {
    state: Mutex<Vec<Todo>>,
}

impl TodoRepository {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(Vec::new())
        }
    }

    pub fn add(&self, todo: Todo) {
        let mut state = self.state.lock().unwrap();

        state.push(todo);
    }

    pub fn list(&self) -> Vec<Todo> {
        let state = self.state.lock().unwrap();

        state.to_vec()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    title: String,
    description: String,
}

#[get("/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Healthy")   
}

#[post("/todos")]
async fn post_todo(req: web::Json<Todo>, data: web::Data<AppState>) -> impl Responder {
    let repository = &data.repository;

    repository.add(Todo {
        title: req.title.to_owned(),
        description: req.description.to_owned(),
    });

    HttpResponse::Created().finish()
}

#[get("/todos")]
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let repository = &data.repository;

    let todos = repository.list();

   let response = serde_json::to_string(&todos).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let app_state = web::Data::new(AppState {
        repository: TodoRepository::new()
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .service(get_health)
            .service(get_todos)
            .service(post_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}