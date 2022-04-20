# [Actix Guide](https://actix.rs/docs/getting-started/)

## Request Handler :
---
* Uses **async** functions
* Accepts *zero* or *more* argumnets
* paramenters can be *extracted* from **FromRequest** trait
* Returns a type that can be converted into an **HttpResponse**

```rust
async fn hello() -> impl Responder
{
    actix_web::HttpResponse::Ok()
    .body("Hello")
}

```
* To attach the routing information to the request handler , there are 2 ways to to do that
* 1) Using `#[get("/")], #[post("/)]` macro on handler
* 2) Can add manually

### HttpResponse
* `impls` **Responder** trait

### App
* Application factory
* Register the *handlers* using
* If _handler_ is attached with route macro then pass handler to `service` factory method
* If __handler is not attached with route then pass to `.route(/path, web::get().to(handler)` 
### HttpServer

* Constructs an application instance for each `thread`
* 


## #[actix_web::main]
    Macro executes the `async main` function within  the `actix` runtime

    
