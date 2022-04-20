# [Application](https://actix.rs/docs/application/)


actix-web servers are built around the `App` instance

### App
1) Registering the **routes** for `resources and middleware`
2) Stores `application state ` shared across all handlers within the same `scope` 
    
    #### Scope
    * Act as a `namespace` for all the `routes`  
    * `web::scope(path)`
    ```rust
         App::new().service(
             # creates the scope called app
             web::scope("/app")
             .route("/index.html", web::get().to(handler))
         )
    ```
  #### State
  * Shared with all `routes` and `resoureces` with in the **same scope** .
  * can be accessed with `web::Data</T>` extractor 
  * Accessible to the `middleware`
  * Any number of state types could be registered within the application 
  ```rust
  App::new()
    .app_data(web::Data::new(?Sized))
  ```
  * `app_data` used to register the state to an application
  * `State` initialized inside the closure that passed to `HttpServer::new()` is local to the worker thread .
  * For global state . State should be initialized outside of `HttpServer::new()`.
  
  ##### Mutable State
  * Shared object should be `Send + Sync`
  * `web::Data` internally uses `Arc` 
  * So need to create `web::Data` before registering with `App`
  * 

  #### Guards and virtual hosting
  *`Guard` is like function that accepts `request object` reference and returns `bool`
  * Guard is object that implements [Guard Trait] (https://docs.rs/actix-web/4/actix_web/guard/trait.Guard.html)
  * Used for playing with `Http Headers`
  #### Configuration
  * `App` and `Scope` provides the `configure` method to configure the scope or app inisde differenct `module` or `binary`
  ```rust
  //any_module.rs
  fn scoped_config(cfg: &mut web::ServiceConfig){
    // now attach the service to the config
    cfg.service(
      web::resource("test/{name or whatever}")
        .route(web::get().to(|| async {HttpResponse::Ok().body("hi")}))
    );
  }
  ``` 
