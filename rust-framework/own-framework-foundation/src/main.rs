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






fn main() {

}


