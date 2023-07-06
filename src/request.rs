use crate::error::ParseError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl HTTPMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::HEAD => "HEAD",
            Self::PUT => "PUT",
            Self::PATCH => "PATCH",
            Self::DELETE => "DELETE",
            Self::CONNECT => "CONNECT",
            Self::OPTIONS => "OPTIONS",
            Self::TRACE => "TRACE",
        }
    }
    pub fn from_str(s: impl Into<String>) -> Option<Self> {
        let s = s.into();
        let s = s.as_str();
        match s {
            "GET" => Some(Self::GET),
            "POST" => Some(Self::POST),
            "HEAD" => Some(Self::HEAD),
            "PUT" => Some(Self::PUT),
            "DELETE" => Some(Self::DELETE),
            "CONNECT" => Some(Self::CONNECT),
            "OPTIONS" => Some(Self::OPTIONS),
            "TRACE" => Some(Self::TRACE),
            "PATCH" => Some(Self::PATCH),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub version: f32,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn new() -> RequestBuilder {
        RequestBuilder::default()
    }
    pub fn from(source: &str) -> Result<Request, ParseError> {
        let lines = source.lines().collect::<Vec<&str>>();
        let perr: ParseError = ParseError::new("Invalid HTTP request");
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body = String::new();

        if lines.is_empty() {
            return Err(perr);
        }

        let fst_line = lines
            .first()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        if fst_line.len() < 2 {
            return Err(perr);
        }

        let md = fst_line.first().unwrap();
        match HTTPMethod::from_str(<&str>::clone(md)) {
            Some(_) => (),
            None => return Err(ParseError::new("Error parsing HTTP method")),
        }

        let pth = fst_line.get(1).unwrap();
        if !pth.contains('/') {
            return Err(ParseError::new("Error parsing HTTP path"));
        }
        let v = fst_line.get(2).unwrap();
        let v = v.split('/').collect::<Vec<&str>>();

        if v.is_empty() {
            return Err(ParseError::new("Error parsing HTTP version"));
        }
        let v = v.get(1).unwrap();
        if v.parse::<f32>().is_err() {
            return Err(ParseError::new("Error parsing HTTP version"));
        }

        let mut b = false;
        for (idx, line) in lines.iter().enumerate() {
            if b {
                body.push_str(line);
                continue;
            }
            if idx == 0 {
                continue;
            };
            if line.is_empty() {
                b = true;
                continue;
            }
            let header = line.split(':').collect::<Vec<&str>>();

            match header.first() {
                Some(_) => (),
                None => return Err(ParseError::new("Error parsing HTTP headers")),
            }
            match header.get(1) {
                Some(_) => (),
                None => return Err(ParseError::new("Error parsing HTTP headers")),
            }
            headers.insert(
                header.first().unwrap().trim().to_string(),
                header.get(1).unwrap().trim().to_string(),
            );
        }

        return Self::new()
            .path(<&str>::clone(pth))
            .method(<&str>::clone(md))
            .version(<&str>::clone(v))
            .headers(headers)
            .body(body)
            .build();
    }
}

// TODO: remove debug trait
#[derive(Default)]
pub struct RequestBuilder {
    method: Option<HTTPMethod>,
    path: Option<String>,
    version: Option<f32>,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    fn headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        let _ = self.headers.insert(headers);
        self
    }
    pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
        let method = HTTPMethod::from_str(method.into());

        // if HTTPMethod::from_str return some value insert it to self.method, otherwise dont
        // change anything (default value of self.method is None)
        if let Some(method) = method {
            let _ = self.method.insert(method);
        }
        self
    }
    pub fn path(&mut self, path: impl Into<String>) -> &mut Self {
        let _ = self.path.insert(path.into());

        self
    }
    pub fn version(&mut self, version: impl Into<String>) -> &mut Self {
        let version = version.into().parse::<f32>();

        // if version.parse returns an error dont change anything (default value is None)
        if let Ok(v) = version {
            let _ = self.version.insert(v);
        }
        self
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
    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        let _ = self.body.insert(body.into());
        self
    }
    pub fn build(&self) -> Result<Request, ParseError> {
        // may be None
        let body: String = match &self.body {
            Some(b) => b.into(),
            None => "".into(),
        };
        // may be None
        let headers: HashMap<String, String> = match self.headers.clone() {
            Some(b) => b,
            None => HashMap::new(),
        };

        // SHOULDN'T BE NONE
        let method: HTTPMethod = match self.method.clone() {
            Some(b) => b,
            None => return Err(ParseError::new("Unspecified HTTP method")),
        };
        let version: f32 = match self.version {
            Some(b) => b,
            None => return Err(ParseError::new("Unspecified HTTP version")),
        };
        let path: String = match &self.path {
            Some(b) => b.into(),
            None => return Err(ParseError::new("Unspecified HTTP path")),
        };
        Ok(Request {
            body,
            method,
            version,
            headers,
            path,
        })
    }
}
