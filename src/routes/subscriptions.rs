use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscriptionFormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<SubscriptionFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}