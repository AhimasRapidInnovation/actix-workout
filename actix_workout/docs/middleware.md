# [Middleware](https://actix.rs/docs/middleware/)

Midlleware can hook into imcoming request and outgoing response

Middleware is invoked on the following actions

* Pre-process the Request
* Post-process a Response
* Modify application state
* Access external services (redis, logging, sessions)

Middleware is registered for each `App`, `Scope`, or `Resources` 


Middleware is a type that implements the 

1) ServiceTrait
2) Transform Trait


## Wrap

`fn` used to register the middleware to an `App`, `scope` or `Resourcecs` 


## wrap_fn

Registers an app-wide function middleware  
Accepts `closure` with `ServiceRequest` and `Service` 

Last registered `wrap_fn` will get executed first

## Default Headers

To ser default response headers `DefaultHeaders` can be used 

>.wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))

