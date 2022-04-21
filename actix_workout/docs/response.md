# [Response](https://actix.rs/docs/response/)

Use `HttpResponse` builder to build the response  
It has several methods including 

* `.body()`

* `.json()`

* `.finish()`

If above methods are called on same builder again it will panic 

## Json Response 

Return value is `web::Json<T>` the should be implemented the `Serialize` trait  

