use actix_web::{get, App, HttpServer, HttpRequest, Responder};
use std::env;

#[get("/")]
async fn get_headers(req: HttpRequest) -> impl Responder {
    let mut headers_info = String::new();

    for (header_name, header_value) in req.headers().iter() {
        if let Ok(name) = header_name.to_str() {
            if let Ok(value) = header_value.to_str() {
                headers_info.push_str(&format!("{}: {}\n", name, value));
            }
        }
    }

    if let Some(peer_addr) = req.peer_addr() {
        headers_info.push_str(&format!("Peer IP: {}\n", peer_addr.ip()));
    } else {
        headers_info.push_str("Cannot determine Peer IP\n");
    }

    headers_info
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");

    HttpServer::new(|| App::new().service(get_headers))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
