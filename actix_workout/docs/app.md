# [Application](https://actix.rs/docs/application/)


actix-web servers are built around the `App` instance

### App
1) Registering the **routes** for `resources and middleware`
2) Stores `application state ` shared across all handlers within the same `scope` 
    
    #### Scope
    * Act as a `namespace` for all the `routes`  

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
  ##### Mutable State
  * Shared object should be `Send + Sync`
  * `web::Data` internally uses `Arc` 
  * So need to create `web::Data` before registering with `App`
  * 

