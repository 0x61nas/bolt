mod utils;
use actix_web::{body, http, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use utils::*;

#[derive(Serialize, Deserialize)]
struct Ping {
    body: String,
}

static VERSION: &str = "0.11.8";
static HELP: &str = r#"
Bolt CLI (Build and test APIs)

Usage:
  bolt [OPTIONS]...
  bolt -h | --help
  bolt -v | --version
Options:
  -h --help      Show this screen.
  -v --version   Show version.
  --reset        Reset dist
    "#;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH,
    OPTIONS,
    CONNECT,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ResponseType {
    TEXT,
    JSON,
}

#[derive(Clone, Serialize)]
struct Response {
    status: u16,
    body: String,
    headers: Vec<Vec<String>>,
    time: u32,
    size: u64,
    response_type: ResponseType,
    request_index: usize,
    failed: bool,
}

impl Response {
    fn new() -> Self {
        Response {
            status: 0,
            body: String::new(),
            headers: Vec::new(),
            time: 0,
            size: 0,
            response_type: ResponseType::TEXT,
            request_index: 0,
            failed: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    url: String,
    method: Method,
    body: String,
    headers: Vec<Vec<String>>,
    request_index: usize,
}

#[actix_web::get("/ping")]
async fn ping(_req: HttpRequest, _body: String) -> HttpResponse {
    let body = Ping {
        body: "pong".to_string(),
    };

    let response = HttpResponse::Ok().json(body);

    return response;
}

#[actix_web::post("/open_link")]
pub async fn open_link(_req: HttpRequest, body: String) -> HttpResponse {
    webbrowser::open(&body).unwrap();

    let response = HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body("done");

    return response;
}

#[actix_web::post("/send_request")]
pub async fn send_request(_req: HttpRequest, body: String) -> HttpResponse {
    // println!("sending request");

    #[derive(Debug, Serialize, Clone, Deserialize)]
    pub struct SendPayload {
        url: String,
        method: Method,
        body: String,
        headers: Vec<Vec<String>>,
        index: usize,
    }

    let payload: SendPayload = serde_json::from_str(&body).unwrap();

    let request = Request {
        url: payload.url,
        method: payload.method,
        body: payload.body,
        headers: payload.headers,
        request_index: payload.index,
    };

    let resp = http_send(request).await;

    let response_body = serde_json::to_string(&resp).unwrap();

    let response = HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body(response_body);

    return response;
}

async fn http_send(mut req: Request) -> Response {
    if !req.url.contains("http") {
        let new_url = "http://".to_string() + &req.url;

        req.url = new_url;
    }

    let mut request = prepare_request(req.clone());

    for h in req.headers {
        if h[0] != "" && h[1] != "" {
            request = request.header(h[0].clone(), h[1].clone());
        }
    }

    let start = get_timestamp();
    let response = request.send().await;
    let end = get_timestamp();

    let mut http_response = match response {
        Ok(resp) => {
            let mut new_response = Response::new();

            new_response.headers = extract_headers(resp.headers());
            new_response.status = resp.status().as_u16();
            new_response.time = (end - start) as u32;
            new_response.body = resp.text().await.unwrap();
            new_response.size = new_response.body.len() as u64;

            if new_response.headers.contains(&vec![
                "content-type".to_string(),
                "application/json".to_string(),
            ]) {
                new_response.response_type = ResponseType::JSON;
            }

            new_response
        }

        Err(err) => {
            let mut err_resp = Response::new();

            err_resp.failed = true;

            err_resp.body = err.to_string();

            err_resp
        }
    };

    http_response.request_index = req.request_index;

    return http_response;
}

#[actix_web::post("/save_state")]
pub async fn save_state(_req: HttpRequest, body: String) -> HttpResponse {
    std::fs::write(get_home() + "state.json", body).unwrap();

    let response = HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body("done");

    return response;
}

#[actix_web::post("/restore_state")]
pub async fn restore_state(_req: HttpRequest) -> HttpResponse {
    let state = std::fs::read_to_string(get_home() + "state.json").unwrap();

    let response = HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body(state);

    return response;
}

pub async fn e404(_req: HttpRequest) -> HttpResponse {
    let body = body::BoxBody::new("Not Found");
    let response: HttpResponse = HttpResponse::new(http::StatusCode::NOT_FOUND).set_body(body);

    return response;
}

#[actix_web::main]
pub async fn launch_server(port: u16) {
    let address = "0.0.0.0";

    let server = HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(restore_state)
            .service(save_state)
            .service(send_request)
            .service(open_link)
            .default_service(web::post().to(e404))
    });

    println!("Starting server on {} port {}", address, port);
    server.bind((address, port)).unwrap().run().await.unwrap();
}

#[actix_web::main]
pub async fn launch_asset_server(port: u16) {
    let address = "0.0.0.0";

    std::thread::spawn(move || {
        println!("opening browser");
        open_browser("http://localhost:".to_string() + &port.to_string());
    });

    let asset_server = HttpServer::new(|| {
        let dist_path = get_dist();

        App::new()
            .service(actix_files::Files::new("/", dist_path).index_file("index.html"))
            .default_service(web::post().to(e404))
    });

    println!("Starting asset server on {} port {}", address, port);
    asset_server
        .bind((address, port))
        .unwrap()
        .run()
        .await
        .unwrap();
}

pub fn start(args: Vec<String>, port: u16) {
    let mut args = args;

    args.remove(0);

    let mut is_tauri = false;
    let mut launch = false;
    let mut reset = false;

    match std::env::var_os("BOLT_DEV") {
        Some(_) => {
            reset = true;
        }
        None => {}
    }

    if args.len() > 0 {
        let flag = args[0].as_str();

        match flag {
            "--reset" => reset = true,

            "-h" | "--help" => {
                println!("{}", HELP);
            }

            "-v" | "--version" => {
                println!("bolt {}", VERSION);
            }

            "--tauri" => {
                is_tauri = true;

                launch = true;
            }

            _ => {
                panic!("unknown flag");
            }
        }
    } else {
        launch = true;
    }

    if reset {
        reset_home();
    }

    if launch {
        verify_home();
        verify_state();

        if !is_tauri {
            verify_dist();
        }

        if !is_tauri {
            std::thread::spawn(move || {
                launch_asset_server(port + 1);
            });
        }

        launch_server(port);
    }
}
