use actix_web::*;

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
