use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

async fn signup(user: web::Json<User>) -> impl Responder {
    println!("User created: {:?}", user);
    HttpResponse::Ok().body("User created successfully\n")
}

async fn login(user: web::Json<User>) -> impl Responder {
    println!("User logged in: {:?}", user);
    HttpResponse::Ok().body("Login successful\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
