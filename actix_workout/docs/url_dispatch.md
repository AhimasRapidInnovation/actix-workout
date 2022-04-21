# [Url Dispatch](https://actix.rs/docs/url-dispatch/)

Maps url to handler code

## Resource Cofiguration

* Act of adding new resource to an application  

* Resource has a name which act as a identifier to be used for `url` generation  

* `App::route()` provides simple way of registering routes  

* `App::service()` adds a single resource to application rotuting table  

```rust
pub fn main() {
    App::new()
        .service(web::resource("/prefix").to(index))
        .service(
            web::resource("/user/{name}")
                .name("user_detail")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::get().to(HttpResponse::Ok))
                .route(web::put().to(HttpResponse::Ok)),
        );
}
```  

## Configuring a Route

* Resource contains a set of routes 

* Each rotues has a set of `guards` and a `handler` 

## Url Pattern Matching 

* foo/{bar}/{tail:.*} to match anything on the tail 

    Example `foo/abc/def/a/b/c  -> Params {'bar': 'abc', 'tail': 'def/a/b/c'}`

## Scoping Routes

Orgainzing routes sharing common roots path  

* /users
* /users/show
* /users/show/{id}

To get this pattern we have to use `scope`

```rust
#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .service(show_users)
                .service(user_detail),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## Match Information

All values representing matched path segments are available in HttpRequest::match_info. Specific values can be retrieved with Path::get().

```rust

use actix_web::{get, App, HttpRequest, HttpServer, Result};

#[get("/a/{v1}/{v2}/")]
async fn index(req: HttpRequest) -> Result<String> {
    let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
    let v2: u8 = req.match_info().query("v2").parse().unwrap();
    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
    Ok(format!("Values {} {} {} {}", v1, v2, v3, v4))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

For type safe path information

```rust
#[get("/{username}/{id}/index.html")] 
async fn index(info: web::Path<(String, u32)>) -> Result<String> {
```

## Generating Resources URL

 HttpRequest.url_for() method to generate URLs based on resource patterns.

```rust
 let url = req.url_for("foo", &["1", "2", "3"])?; //  http://example.com/test/1/2/3
  App::new()
            .service(
                web::resource("/test/{a}/{b}/{c}")
                    .name("foo") // <- set resource name, then it could be used in `url_for`
                    .guard(guard::Get())
                    .to(HttpResponse::Ok),
            )
            .service(index)
```

## External Resources

* Resources that are valid URL can be registered as external resources

* Useful for `url generation` purpose  

```rust
let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap();
assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");

// fn main()

 App::new()
    .service(index)
    .external_resource("youtube", "https://youtube.com/watch/{video_id}")

```

## Path Normalization

Normalization means 

* To add a trailing slash to the path.
* To replace multiple slashes with one.

## Custom route guard

Guards is like a function that accepts a request reference and returns true or false

Guard is any type that implement `Guard` trait

```rust
use actix_web::{guard::{Guard, GuardContext}};

struct ContentTypeHeader;

impl Guard for ContentTypeHeader {

    fn check(&self, req: &GuardContext) -> bool {
        req
            .head()
            .headers()
            .contains_key(http::header::CONTENT_TYPE)
    }
}


// in main fn 
  App::new().route(
            "/",
            web::route().guard(ContentTypeHeader).to(HttpResponse::Ok),
        )
```

Guards can not access or modify the request object 

METHOD_NOT_ALLOWED response to all except GET request

* Not

* Any (or)

* All (and)


```rust
 web::route()
                .guard(guard::Not(guard::Get()))
                .to(HttpResponse::MethodNotAllowed),

guard::Any(guard::Get()).or(guard::Post())


guard::All(guard::Get()).and(guard::Header("content-type", "plain/text"))

```



## Default Not found

`App::default_service()` 

```rust
 App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
```

