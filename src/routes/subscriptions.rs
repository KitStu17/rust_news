use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

// 무조건 200 OK 를 반환하는 api
pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>
) -> HttpResponse {
    // 무작위 고유 식별자 생성
    let req_id = Uuid::new_v4();
    log::info!("request_id {} - Adding '{}' '{}' as a new subscriber.",
        req_id,
        form.email,
        form.name);
    log::info!(
        "request_id {} - Saving new subscriber detrails in the database",
        req_id
    );
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("request_id {} - New subscriber details have been saved", req_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            // println!("Faile to execute query: {}", e);
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}