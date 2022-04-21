# [Satic Files](https://actix.rs/docs/static-files/)

## IndiviApp::new().route("/{filename:.*}", web::get().to(index)dual File

`Named File` type can be used to send the file  

```rust
 Ok(NamedFile::open(path)?)

 // in main httpServer instance factory
 App::new().route("/{filename:.*}", web::get().to(index)
```

`{.*}` used for match the path tail  

## Directory

> Files type  

Serve files from specific `directories` and `subdirectories`  

`Files` must be registered with `App::service` 

```rust
App::new().service(fs::Files::new("/static", ".").show_files_listing())
```

`show_file_listing()` is to enable sub directories  

## Configuration

`NamedFile` options  

* set_content_disposition()
* use_etag() Etag type
* use_last_modified() -> last modified time 

```rust
let file = fs::NamedFile::open(path)?;
Ok(file
    .use_last_modified(true)
    .set_content_disposition(ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![],
    }))
```

