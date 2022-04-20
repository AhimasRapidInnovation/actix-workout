# [Server](https://actix.rs/docs/server/)

[HttpServer type](https://docs.rs/actix-web/4.0.1/actix_web/struct.HttpServer.html) is responsible for serving `http` requests

## HttpServer

* Accepts application factory
* Factory must be `send + Sync`

### bind(Addr)

* socket address tuples or string

### run() -> Server

* Returns the `server` instance

* Server must be `await` to start processing requests

* `Server` will run until it receives `shutdown` signals

### MultiThreading

* `HttpServer` automatically starts number of threads by default <small> num_cpu() </small>

* Can override usign `HttpServer::workers()` method

> HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok))).workers(4);

* Workers receives seperate application instance to handle the requests

* Application state is not shared between threads , App state do not need `Send` or `Sync` but factory needs it.

Use `tokio` for  long running io operations so the worker thread do not block the execution

## TLS / HTTPS

* use `rustls` and `openssl`

```toml
[dependencies]
actix-web = { version = "4", features = ["openssl"] } openssl = { version = "0.10" }
```

## Keep Alive

* Defined by server setting

* Can use `Duration::from_secs(75)` or `KeepAlive::Timeout(75)

* `None` or `KeepAlive::Disabled` disables `keep-alive`

```rust
HttpServer::new(app).keep_alive(Duration::from_secs(75))
HttpServer::new(app).keep_alive(KeepAlive::Os)
```

## Graceful Shutdown

* `HttpServer` supports graceful shutdown

* Any workers still alive after the timeout are force-dropped 

* Can use `HttpServer::shutdown_timeout()` method

* To disable the `signals` use `HttpServer::disable_signals()` method

  