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

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

// 무조건 200 OK 를 반환하는 api
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
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
            // 새로만든 subscribe api 처리, post 요청으로 받는다.
            .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();
        
        Ok(server)
}