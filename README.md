# http_parser
Converts raw request to `Request` 

### Parse the raw http request to `Request`
```rust
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        let mut tcp_stream = stream.unwrap();
        let mut buffer = [0; 16384];
        tcp_stream.read(&mut buffer).unwrap();

        let raw_request = String::from_utf8_lossy(&buffer);

        match Request::from(&raw_request) {
            Ok(_) => {
                tcp_stream
                    .write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Hello World</h1>",
                    )
                    .unwrap();
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
```
