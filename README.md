# http_parser
Converts raw request to `Request` and build `Response`s 

### Parse the raw http request to `Request`
```rust
    for stream in listener.incoming(){
        let mut tcp_stream = stream.unwrap();
        
        let mut buffer = [0; 16384];
        tcp_stream.read(&mut buffer).expect("Error reading stream");

        let raw_request = String::from_utf8_lossy(&buffer).to_string();
        let request = simple_http_parser::Request::from(raw_request);
```

### And now you can use `Request` properties to build a `Response` and send it

```rust
        let mut response = simple_http_parser::Response::new();

        if request.path == "/" {
            response.set_body(simple_http_parser::Body::Text("Hello, World!".to_owned()));
        } else if request.path == "/favicon.ico" {
            response.set_header("Content-Type", "image/x-icon");
            
            let mut file = std::fs::File::open("./favicon.png").unwrap();
            let mut contents = vec![];
            file.read_to_end(&mut contents).unwrap();

            response.set_header("Content-Length", format!("{}", contents.len()).as_str());
            response.set_body(simple_http_parser::Body::Bytes(contents));
        } else {
            response.set_header("Content-Type", "application/json");
            response.set_body(simple_http_parser::Body::Text(format!("{{\n\t\"actualPath\":\"{}\"\n}}", request.path)));
        }

        tcp_stream.write_all(&response.build()).unwrap()
    }
```
