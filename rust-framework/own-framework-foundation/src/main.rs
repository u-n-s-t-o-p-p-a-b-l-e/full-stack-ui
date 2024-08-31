use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

type HandlerFunction = fn(&Request) -> Response;

#[allow(dead_code)]
#[derive(Clone)]
struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[allow(dead_code)]
struct Response {
    status_code: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Router {
    routes: HashMap<String, HandlerFunction>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, path: &str, handler: HandlerFunction) {
        self.routes.insert(path.to_string(), handler);
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.path) {
            Some(handler) => handler(request),
            None => not_found(),
        }
    }
}

fn parse_request(mut stream: &TcpStream) -> Request {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer[..]);
    let mut lines = request_str.lines();

    let first_line = lines.next().unwrap_or("");
    let mut parts =first_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();
}






fn main() {

}


