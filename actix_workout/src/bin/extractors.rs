use actix_web::{get,HttpServer, App, web, Result};

use serde::{Deserialize};

#[derive(Deserialize)]
struct Info {
    user_id: Option<u32>,
    friend: Option<String>,
}

#[derive(Deserialize)]
struct QueryInfo{
    name: String
}

#[get("/query")]
async fn query(info: web::Query<QueryInfo>) -> Result<String> {
    Ok(format!("Welcome {:?}", info.name))
}


// async fn check_builder() -> String {

//     let mut builder = HttpResponse::Ok()
//                         .content_type()
//     "".into()
// }


#[get("/users/{user_id}/{name}")]
async fn hello(path: web::Path<(i32, String)>) -> Result<String>
{
    let (user_id, name) = path.into_inner();
    Ok(format!("Welcome {} you id is {} ", name, user_id))
}



#[get("/index/{user_id}/{friend}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend.as_ref().unwrap_or(&"Friend not set".to_string()).clone(), info.user_id.unwrap_or(0000)
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
                                .service(hello)
                                .service(index)
                                .service(query)
                                )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}