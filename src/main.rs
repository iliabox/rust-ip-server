use actix_web::{get, App, HttpServer, HttpRequest, Responder};
use std::env;

#[get("/")]
async fn get_ip(req: HttpRequest) -> impl Responder {
    let mut response = String::new();

    let client_ip = if let Some(cf_ip) = req.headers().get("CF-Connecting-IP") {
        cf_ip.to_str().unwrap_or("Unknown CF-Connecting-IP")
    } else if let Some(forwarded) = req.headers().get("X-Forwarded-For") {
        forwarded.to_str().unwrap_or("Unknown X-Forwarded-For").split(',').next().unwrap_or("Unknown").trim()
    } else if let Some(peer_addr) = req.peer_addr() {
        let ip_str = peer_addr.ip().to_string();
        ip_str.as_str()
    } else {
        "Cannot determine IP"
    };

    response.push_str(&format!("IP: {}\n\n", client_ip));
    response.push_str("Request Headers:\n");

    for (key, value) in req.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            response.push_str(&format!("{}: {}\n", key, value_str));
        } else {
            response.push_str(&format!("{}: <non-UTF8 value>\n", key));
        }
    }

    response
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
