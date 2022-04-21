# [Request](https://actix.rs/docs/request/)


Manually load the payload into the memory 

Loading the request body


```rust
#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144;

#[post("/")]
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

```

## Content Encoding

If request header contain `Content-Encoding` header the request payload is decompressed according to header value 


## Streaming Request

`HttpRequest` is a stream of `Bytes` object can  be used to read the request body payload

```rust
use actix_web::{get, web, Error, HttpResponse};
use futures::StreamExt;

#[get("/")]
async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    Ok(HttpResponse::Ok().finish())
}

```