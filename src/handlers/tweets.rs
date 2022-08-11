use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/tweets")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Tweet#index")
}

#[post("/tweets")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Tweet#new")
}

#[get("/tweets/{id}")]
pub async fn show(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Tweet#show {}", id))
}

#[put("/tweets/{id}")]
pub async fn update(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Tweet#edit {}", id))
}

#[delete("/tweets/{id}")]
pub async fn destroy(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Tweet#delete {}", id))
}
