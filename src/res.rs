use std::{io::Write, net::TcpStream};

#[derive(Clone, Debug)]
pub struct Response {
    pub version: f32,
    pub status: i32,
    pub status_message: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl Response {
    pub fn new() -> Response {
        return Response {
            version: 1.1,
            status: 200,
            status_message: "OK".to_owned(),
            headers: vec!["Content-Type: text/plain".to_owned()],
            body: "".to_owned(),
        };
    }

    pub fn send(&self, mut stream: &TcpStream) {
        let mut response = format!(
            "HTTP/{} {} {}\r\n",
            self.version, self.status, self.status_message
        );
        for header in self.headers.clone() {
            response.push_str(format!("{}\r\n", header).as_str())
        }
        response.push_str(format!("\r\n{}\r\n", self.body).as_str());
        stream.write_all(response.as_bytes()).unwrap()
    }
}
