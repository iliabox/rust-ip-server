use actix_web::{get, App, HttpServer, HttpRequest, Responder};
use std::env;

#[get("/")]
async fn get_ip(req: HttpRequest) -> impl Responder {
    if let Some(peer_addr) = req.peer_addr() {
        format!("Your IP: {}", peer_addr.ip())
    } else {
        "Cannot determine IP".to_string()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    HttpServer::new(|| App::new().service(get_ip))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
