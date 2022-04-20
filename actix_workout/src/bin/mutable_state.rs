use actix_web::{get,HttpServer,App, web, Responder, HttpResponse};

use std::sync::Mutex;



fn scoped_config(cfg: &mut web::ServiceConfig){
    // now attach the service to the config
    cfg.service(
      web::resource("test/{name}")
        .route(web::get().to(|| async {HttpResponse::Ok().body("From scoped config")}))
    );
}

struct MutableState {
    counter: Mutex<usize>, // since `web::Data` already uses Arc we do no need to use Sync 
}


#[get("/")]
async fn index(data: web::Data<MutableState>) -> impl Responder {

    match data.counter.lock()
    {
        Ok(mut guard) => {
            (*guard)+= 1;

            format!("count is {}", *guard)
        },
        Err(e) => format!("Got Error {}", e)
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(
        MutableState {counter: Mutex::new(0)}
    );
    HttpServer::new(move || {

        App::new()
            .app_data(state.clone())
            .configure(scoped_config)
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
    
    Ok(())
}

