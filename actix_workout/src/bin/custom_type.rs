

use actix_web::{get, body,App, HttpServer, Responder, HttpResponse, http::header::ContentType};

use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize)]
struct CustomResponse{
    field : String
}


impl Responder for CustomResponse {
    type Body = body::BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        // serialize the self 
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
        
    }
}

#[get("/")]
async fn hello() -> CustomResponse 
{
    CustomResponse{field : "CustomResponse is on the way".into()}
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}