use actix_web::{get,web,HttpServer, App, guard, HttpResponse};






#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new( || {
        App::new()
                .service(
                    web::scope("/")
                            // .app_data(web::Data::new(1))
                            .guard(guard::Host("TestHost"))
                            .route("", web::to(|| async {HttpResponse::Ok().body("From test host")}))
                )
                .service(
                    web::scope("/")
                                .guard(guard::Host("NonTestHost"))
                                .route("", web::to(|| async {HttpResponse::Ok().body("From Non test host")}))
                )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}