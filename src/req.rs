use std::io::Read;
use std::net::TcpStream;
use std::ops::Index;

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub version: f32,
    pub headers: Vec<String>,
    pub body: String,
}

impl Request {
    pub fn from(mut stream: &TcpStream) -> Request {
        let mut buffer = [0; 16384];
        stream.read(&mut buffer).expect("Error reading stream");
        let mut req = String::new();
        req.push_str(&String::from_utf8_lossy(&buffer));

        let mut request: Vec<&str> = req.split("\r\n").collect();

        let start_line: Vec<&str> = request.index(0).split(" ").collect();
        request.remove(0); // remove the start line from the request vector

        let method = match start_line.index(0).to_uppercase().as_str() {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            "HEAD" => HTTPMethod::HEAD,
            "PUT" => HTTPMethod::PUT,
            "DELETE" => HTTPMethod::DELETE,
            "CONNECT" => HTTPMethod::CONNECT,
            "OPTIONS" => HTTPMethod::OPTIONS,
            "TRACE" => HTTPMethod::TRACE,
            _ => HTTPMethod::GET 
        };

        let path = start_line.index(1).to_string();

        let version = start_line.index(2)
            .split("/").collect::<Vec<&str>>().index(1)
            .parse::<f32>().expect("Error reading HTTP version");

        let mut body = request.last().unwrap().to_string();
        request.pop();
        body = body.replace("\0", "");

        // remove the \r\n\r\n between the headers and the body
        if request.last().unwrap() == &"" {
            request.pop();
        }

        let mut headers = Vec::new();
        for header in request {
            headers.push(header.to_string())
        }

        Request {
            method,
            path,
            version,
            body,
            headers,
        }
    }
}
