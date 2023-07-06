use std::collections::HashMap;

pub enum Body {
    Text(String),
    Bytes(Vec<u8>),
}

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    status_message: String,
    pub headers: HashMap<String, String>,
    pub version: f32,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new() -> ResponseBuilder {
        return ResponseBuilder::default();
    }
}

#[derive(Default)]
pub struct ResponseBuilder {
    pub status: Option<u16>,
    pub headers: Option<HashMap<String, String>>,
    pub version: Option<f32>,
    pub body: Option<Vec<u8>>,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        return Self::default();
    }

    fn status_message(&mut self) -> String {
        let msg = match self.status.unwrap() {
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",
            103 => "Early Hints",
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            207 => "Multi-Status",
            208 => "Already Reported",
            218 => "This is fine",
            226 => "IM Used",
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Lenght Required",
            412 => "Precondition Failed",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            416 => "Range Not Satisfiable",
            417 => "Expectation Failed",
            418 => "I'm a teapot",
            421 => "Misdirected Request",
            422 => "Unprocessable Entity",
            423 => "Locked",
            424 => "Failed Dependency",
            426 => "Upgrade Required",
            428 => "Precondition Required",
            429 => "Too Many Requests",
            431 => "Request Header Fields Too Large",
            451 => "Unavailable For Legal Reasons",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",
            506 => "Variant Also Negotiates",
            507 => "Insufficient Storage",
            508 => "Loop Detected",
            510 => "Not Extended",
            511 => "Network Authentication Required",
            _ => "Internal Server Error",
        };
        return msg.to_string();
    }
    pub fn status_code(&mut self, status_code: impl Into<u16>) -> &mut Self {
        let _ = self.status.insert(status_code.into());
        return self;
    }
    pub fn header(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        if let Some(ref mut headers) = self.headers {
            headers.insert(key.into(), value.into());
        } else {
            let header = HashMap::from([(key.into(), value.into())]);
            let _ = self.headers.insert(header);
        }
        self
    }
    pub fn version(&mut self, version: impl Into<f32>) -> &mut Self {
        let _ = self.version.insert(version.into());
        return self;
    }
    pub fn body(&mut self, body: Body) -> &mut Self {
        match body {
            Body::Text(b) => self.body.insert(Vec::from(b.as_bytes())),
            Body::Bytes(b) => self.body.insert(b),
        };
        return self;
    }
    pub fn build(&mut self) -> Response {
        let status = match self.status {
            Some(s) => s,
            None => 200_u16,
        };
        let body = match self.body.clone() {
            Some(b) => b,
            None => Vec::new(),
        };
        let version = match self.version {
            Some(v) => v,
            None => 1.1_f32,
        };
        let headers = match self.headers.clone() {
            Some(h) => h,
            None => HashMap::new(),
        };

        // only call berfore verifing that self.status isn't None
        let status_message = self.status_message();
        return Response {
            status,
            status_message,
            body,
            version,
            headers,
        };
    }
}
