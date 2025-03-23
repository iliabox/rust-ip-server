use actix_web::{get, App, HttpServer, HttpRequest, Responder};
use std::env;

#[get("/")]
async fn get_ip(req: HttpRequest) -> impl Responder {
    if let Some(cf_ip) = req.headers().get("CF-Connecting-IP") {
        if let Ok(ip_str) = cf_ip.to_str() {
            return format!("IP: {}", ip_str);
        }
    }

    if let Some(forwarded) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            let client_ip = forwarded_str.split(',').next().unwrap_or("Unknown").trim();
            return format!("IP: {}", client_ip);
        }
    }

    if let Some(peer_addr) = req.peer_addr() {
        format!("IP: {}", peer_addr.ip())
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

    println!("Server running on port {}", port);

    HttpServer::new(|| App::new().service(get_ip))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
