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

    let mut headers = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut parts = line.splitn(2, ": ");
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    Request {
        method,
        path,
        headers,
        body: Vec::new(),
    }
}

fn not_found() -> Response {
    Response {
        status_code: 404,
        headers: HashMap::new(),
        body: b"404 Not Found".to_vec(),
    }
}

fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let request = parse_request(&stream);
    let response = router.handle_request(&request);

    let  response_string = format!(
        "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n",
        response.status_code,
        response.body.len()
    );

    stream.write(response_string.as_bytes()).unwrap();
    stream.write(&response.body).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let mut router = Router::new();

    router.add_route("/", |_| Response {
        status_code: 200,
        headers: HashMap::new(),
        body: b"Welcome to the home page!".to_vec(),
    });

    router.add_route("/about", |_| Response {
        status_code: 200,
        headers: HashMap::new(),
        body: b"This is the about page.".to_vec(),
    });

    let router = Arc::new(router);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let router = Arc::clone(&router);

        thread::spawn(move || {
            handle_connection(stream, router);
        });
    }
}


