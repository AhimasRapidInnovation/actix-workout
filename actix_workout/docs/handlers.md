# [Request Handlers](https://actix.rs/docs/handlers/)

Request handling happens in 2 stage

1) Handler object is called and returning any object that impls `Responder` trait 
2) `respond_to()` is called on the returned object converting itself to a `HttpResponse` or `Error`

```rust
web::Bytes::from_static(b"hi there") -> impls Responder

async fn index(req: HttpRequest) -> Box<Future<Item=HttpReponse, Error=Error>>
```

## Response with custom types

Type needs to implement `Responder` trait

### Responder trait 

```rust

trait Responder{
    type Body;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body>;
}

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

```

## Streaming response body

Body must be implement `streaming` trait eg `Stream<Item=Bytes, Error=Error>`
Use `HttpResponse::streaming(<Stream Trait>)` method to 