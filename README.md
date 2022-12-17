# http_parser
Converts raw request to `Request` and build `Response`s 

### Parse the raw http request to `Request`
```rust
    for stream in listener.incoming(){
        let mut tcp_stream = stream.unwrap();
        let request = http_parser::Request::from(&tcp_stream);
```

### And now you can use `Request` properties to build a `Response` and send it

```rust
        let request = http_request_parser::Request::from(&tcp_stream);
        let mut response = http_request_parser::Response::new();

        if request.path == "/" {
            response.body = "Hello, World!".to_owned();
        } else {
            response.headers = vec!["Content-Type: application/json".to_owned()];
            response.body = format!("{{\n\t\"actualPath\":\"{}\"\n}}", request.path);
        }
        response.send(&tcp_stream)
    }
```