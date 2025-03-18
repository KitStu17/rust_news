use crate::routes::{health_check, subscribe, greet};

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use std::net::TcpListener;


pub fn run(listener: TcpListener,
        // postgresql 과 연결하기 위한 매개변수
        db_pool: PgPool
        ) -> Result<Server, std::io::Error> {

    let pg_pool = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            // App 에 대해 wrap 메서드로 미들웨어 추가 
            .wrap(Logger::default())
            // route() -> Route 구조체 인스턴스
            // web::get() = Route::new().guard(guard::GET()) get요청일 때만 처리
            .route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            // 새로만든 subscribe api 처리, post 요청으로 받는다.
            .route("/subscriptions", web::post().to(subscribe))
            // connection의 포인터 사본을 애플리케이션 상태의 일부로 등록
            .app_data(pg_pool.clone())
        })
        .listen(listener)?
        .run();
        
        Ok(server)
}