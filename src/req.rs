use std::io::Read;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String, //path type gives errors
    pub version: f32,
    pub headers: Vec<String>,
    pub body: String,
}
impl Request {
    pub fn from(mut stream: &TcpStream) -> Request {
        let mut buffer = [0; 16384];
        stream.read(&mut buffer).expect("Error reading stream");

        let mut buffer_v = Vec::from_iter(buffer.iter().copied());

        buffer_v.pop();
        let raw_req = std::str::from_utf8(&buffer_v).unwrap().to_string();
        let req_handler: Vec<&str> = raw_req.split("\r\n").collect();
        let mut req: Vec<String> = vec![];

        for i in &req_handler {
            req.push(i.to_string())
        }

        let line_one: Vec<&str> = req[0].split(' ').collect();

        let method = line_one[0].to_string();
        let path = line_one[1].to_string();

        let mut trash: Vec<&str> = line_one[2].split('/').collect(); //useless
        let version = trash[1].parse::<f32>().unwrap();

        trash = req[req.len() - 1].split('\0').collect();
        let body = trash[0].to_string();

        req.remove(0);
        req.remove(req.len() - 1);
        req.remove(req.len() - 1);

        let headers: Vec<String> = req.to_vec(); // minus 2 'cause the last is the body and penultimate is "\r\n\r\n"

        Request {
            method,
            path,
            version,
            body,
            headers,
        }
    }
}
