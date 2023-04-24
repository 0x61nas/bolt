use std::fmt;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH,
    OPTIONS,
    CONNECT,
    COPY,
}

impl From<usize> for HttpMethod {
    fn from(index: usize) -> Self {
        match index {
            0 => HttpMethod::GET,
            1 => HttpMethod::POST,
            2 => HttpMethod::PUT,
            3 => HttpMethod::DELETE,
            4 => HttpMethod::HEAD,
            5 => HttpMethod::PATCH,
            6 => HttpMethod::OPTIONS,
            7 => HttpMethod::CONNECT,
            8 => HttpMethod::COPY,
            _ => panic!("Invalid index for HttpMethod"),
        }
    }
}

impl HttpMethod {
    pub fn count() -> usize {
        9
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpMethod::COPY => write!(f, "COPY"),
            HttpMethod::CONNECT => write!(f, "CONNECT"),
        }
    }
}

impl From<String> for HttpMethod {
    fn from(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "get" => HttpMethod::GET,
            "post" => HttpMethod::POST,
            "put" => HttpMethod::PUT,
            "delete" => HttpMethod::DELETE,
            "head" => HttpMethod::HEAD,
            "patch" => HttpMethod::PATCH,
            "options" => HttpMethod::OPTIONS,
            "copy" => HttpMethod::COPY,
            "connect" => HttpMethod::CONNECT,
            _ => panic!("Invalid value for HttpMethod"),
        }
    }
}

impl From<HttpMethod> for String {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::PUT => "PUT".to_string(),
            HttpMethod::DELETE => "DELETE".to_string(),
            HttpMethod::HEAD => "HEAD".to_string(),
            HttpMethod::PATCH => "PATCH".to_string(),
            HttpMethod::OPTIONS => "OPTIONS".to_string(),
            HttpMethod::COPY => "COPY".to_string(),
            HttpMethod::CONNECT => "CONNECT".to_string(),
        }
    }
}

pub enum RequestTabs {
    Body,
    Params,
    Headers,
}

impl From<u8> for RequestTabs {
    fn from(value: u8) -> Self {
        match value {
            1 => RequestTabs::Body,
            2 => RequestTabs::Params,
            3 => RequestTabs::Headers,
            _ => panic!("Invalid value for RequestTabs"),
        }
    }
}

impl From<RequestTabs> for u8 {
    fn from(tab: RequestTabs) -> Self {
        match tab {
            RequestTabs::Body => 1,
            RequestTabs::Params => 2,
            RequestTabs::Headers => 3,
        }
    }
}

pub enum ResponseTabs {
    Body,
    Headers,
}

impl From<u8> for ResponseTabs {
    fn from(value: u8) -> Self {
        match value {
            1 => ResponseTabs::Body,
            2 => ResponseTabs::Headers,
            _ => panic!("Invalid value for ResponseTabs"),
        }
    }
}

impl From<ResponseTabs> for u8 {
    fn from(tab: ResponseTabs) -> Self {
        match tab {
            ResponseTabs::Body => 1,
            ResponseTabs::Headers => 2,
        }
    }
}