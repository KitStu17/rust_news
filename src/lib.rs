use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // route() -> Route 구조체 인스턴스
            // web::get() = Route::new().guard(guard::GET()) get요청일 때만 처리
            .route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
        })
        .listen(listener)?
        .run();
        
        Ok(server)
}