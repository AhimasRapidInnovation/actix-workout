# [Extractors](https://actix.rs/docs/extractors/)

Type safe request information access 
[Built in extractor implementation](https://actix.rs/actix-web/actix_web/trait.FromRequest.html#implementors)


## Path

Information that is extracted from from `request's` path
Parts of path that are extractable are called `dynamic segments` and marked with `{}`

`/users/{user_id}/{friend}` and here `user_id` and `friend` can be extracted as a tuple in order `Path<(i32,String)>`

It is possible to extract `path` information to a type that implements the `Deserialize` trait

```rust
#[derive(Deserialize)]
struct Info {
    user_id: Option<u32>,
    friend: Option<String>,
}
#[get("/users/{user_id}/{friend}")] 
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}
```

Non type safe alternative to access the segments

```rust
#[get("/users/{user_id}/{friend}")]
async fn index(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}
```

## Query

`Query<T>` provides extraction functionality for the request query paramenters
uses `serde_urlencoded` crate

```rust
#[derive(Deserialize)]
struct QueryInfo{
    name: String
}

#[get("/query)]
async fn query(info: web::Query<QueryInfo>) -> Result<String> {
    Ok(format!("Welcome {:?}", info.name))
}

```

## Json

`Json<T>` allows `deserialization` of a request body into a struct
to extract from the request body `T` must implement `Deserialize` trait

```rust
#[derive(Deserialize)]
struct Info {
    username: String,
}

#[get("/")]
async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}
```

To configure the extraction process use 

```rust
let json_config = web::JsonConfig::default()
            //limits 4kb
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
App::new()
    .app_data(json_config)
    ..
```

## URL-Encoded Forms

Much like [Json](#json)
[FormConfig](https://docs.rs/actix-web/4.0.1/actix_web/web/struct.FormConfig.html) allows configuring the extaction process

```rust
#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[post("/")]
async fn index(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.username))
}
```

## Data

To get an access to application state 

## actix_web::web::Bytes 

Convert request's payload into `Bytes`


