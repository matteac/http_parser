#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String, //path type gives errors
    pub version: f32,
    pub headers: Vec<String>,
    pub body: String,
}
