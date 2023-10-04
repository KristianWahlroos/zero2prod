use actix_web::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
