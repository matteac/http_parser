# http_parser
Converts raw request to struct `Request` with it's properties

### The `Request` properties are:
 * `method` of type `String`
 * `path` of type `String`
 * `version` of type `f32`
 * `headers` of type `Vec<String>`
 * `body` of type `String`

### Parse the raw string to `Request`
```rust
    for stream in listener.incoming(){
        let tcp_stream = stream.unwrap();
        let request = http_parser::req(&tcp_stream);
```

### And now you can use `Request` properties to respond

```rust
        if request.path == "/" {
            let response = format!(
                "HTTP/1.1 {}\n{}\r\n\r\nHi! you're in {}\n",
                "200 Ok", "Content-Type: text/plain", request.path
            );
            stream.write_all(response.as_bytes()).unwrap()
        } else {
            let response = format!(
                "HTTP/1.1 {}\n{}\r\n\r\nCannot {} {}\n",
                "404 Not Found", "Content-Type: text/plain", request.method,
                request.path
            );
            stream.write_all(response.as_bytes()).unwrap ()
        }

    }
```