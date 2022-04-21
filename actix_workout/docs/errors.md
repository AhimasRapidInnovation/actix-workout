# [Errors](https://actix.rs/docs/errors/)

Actix uses its own `Error` type `actix_web::error::Error` and `actix_web::error::ResponseError` trait for error handling from web handlers

Most of the std::error can be converted into ResponseError by   `From` trait

```rust
pub trait ResponseError {
    fn error_response(&self) -> Response<Body>;
    fn status_code(&self) -> StatusCode;
}
```

## Custom Error Response

Response error default impls is 500(internal server error)

Can use struct as well as enum
For struct use `derive_more` crate

```rust
impl error::ResponseError for ErrorEnum {
    
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

}

```

## Error Helpers

Helper function useful for for generating specific HTTP error codes from other errors  

```rust
#[derive(Debug)]
struct MyError {
    name: &'static str,
}

#[get("/")]
async fn index() -> Result<&'static str> {
    let result: Result<&'static str, MyError> = Err(MyError { name: "test error" });
    //  
    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}
```

## Error Logging

Logs all the errors `WARN` log level  

> To log the backtrace RUST_BACKTRACE should be enabled as well as RUST_LOG is set to debug
> RUST_BACKTRACE=1 RUST_LOG=actix_web=debug cargo run

## Recommended Practice in Error Handling

Divide the errors into two parts

1) User facing errors
2) System facing errors

Mapping the internal error to user faced error

```rust
#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/")]
async fn index() -> Result<&'static str, UserError> {
    do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}
```

## Error Logging using loggers

`middleware::Logger` depends on `env_logger` and `log`  

Env Logger:
    Logger that can be configured via environment variables 
