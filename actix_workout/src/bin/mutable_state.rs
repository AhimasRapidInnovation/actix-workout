use actix_web::{get,HttpServer,App, web, Responder};

use std::sync::Mutex;


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
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
    
    Ok(())
}

