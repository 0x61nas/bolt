use crate::utils::*;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri_sys::tauri;
use yew::{html::Scope, Component, Context, Html};

mod process;
mod style;
mod utils;
mod view;

// http://localhost:2000/ping

// #[wasm_bindgen(module = "/script.js")]
// extern "C" {}

// TODO: enter button to send
// TODO: Copy response body button 

// Define the possible messages which can be sent to the component
#[derive(Clone)]
pub enum Msg {
    SelectedMethod(Method),
    SendPressed,
    ReqBodyPressed,
    ReqHeadersPressed,
    ReqParamsPressed,

    RespBodyPressed,
    RespHeadersPressed,

    AddHeader,
    RemoveHeader(usize),

    AddParam,
    RemoveParam(usize),

    ReceivedResponse,

    MethodChanged,
    UrlChanged,
    BodyChanged,
    HeaderChanged(usize),
    ParamChanged(usize),

    AddRequest,
    RemoveRequest(usize),
    SelectRequest(usize),

    AddCollection,
    RemoveCollection(usize),
    AddToCollection(usize),

    SelectFromCollection(usize, usize),
    RemoveFromCollection(usize, usize),

    ToggleCollapsed(usize),

    Update,
    HelpPressed,
    SwitchPage(Page),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Collections,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ResponseType {
    TEXT,
    JSON,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

impl Method {
    fn to_string(&self) -> String {
        match self {
            Method::GET => "get".to_string(),
            Method::POST => "post".to_string(),
            Method::PUT => "put".to_string(),
            Method::DELETE => "delete".to_string(),
            Method::HEAD => "head".to_string(),
            Method::PATCH => "patch".to_string(),
            Method::OPTIONS => "options".to_string(),
            Method::CONNECT => "connect".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoltApp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response {
    status: u16,
    body: String,
    headers: Vec<Vec<String>>,
    time: u32,
    size: u64,
    response_type: ResponseType,
    request_index: usize,
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
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Request {
    url: String,
    body: String,
    headers: Vec<Vec<String>>,
    params: Vec<Vec<String>>,
    method: Method,

    response: Response,

    // META
    name: String,
    request_index: usize,

    req_tab: u8,
    resp_tab: u8,
}

impl Request {
    fn new() -> Request {
        Request {
            url: String::new(),
            body: String::new(),
            headers: vec![vec![String::new(), String::new()]],
            params: vec![vec![String::new(), String::new()]],
            method: Method::GET,

            response: Response::new(),

            // META
            name: "New Request ".to_string(),
            request_index: 0,

            req_tab: 1,
            resp_tab: 1,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Collection {
    name: String,
    requests: Vec<Request>,
    collapsed: bool,
}

impl Collection {
    fn new() -> Collection {
        Collection {
            name: "New Collection ".to_string(),
            requests: vec![],
            collapsed: false,
        }
    }
}

pub struct BoltState {
    bctx: BoltContext,
}

#[derive(Clone)]
pub struct BoltContext {
    link: Option<Scope<BoltApp>>,

    page: Page,
    main_current: usize,
    col_current: Vec<usize>,

    main_col: Collection,
    collections: Vec<Collection>,
    // resized: bool,
}

impl BoltContext {
    fn new() -> Self {
        let bctx = BoltContext {
            link: None,

            main_col: Collection::new(),
            collections: vec![],
            page: Page::Home,

            main_current: 0,
            col_current: vec![0, 0],
            // resized: false,
        };

        return bctx;
    }
}

unsafe impl Sync for BoltApp {}
unsafe impl Send for BoltApp {}
unsafe impl Sync for BoltState {}
unsafe impl Send for BoltState {}

impl BoltState {
    fn new() -> Self {
        Self {
            bctx: BoltContext::new(),
        }
    }
}

// Create a shared global state variable
lazy_static::lazy_static! {
    static ref GLOBAL_STATE: Arc<Mutex<BoltState>> = Arc::new(Mutex::new(BoltState::new()));
}

impl Component for BoltApp {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        disable_text_selection();

        let mut state = GLOBAL_STATE.lock().unwrap();
        state.bctx.link = Some(ctx.link().clone());

        state.bctx.main_col.requests.push(Request::new());

        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut state = GLOBAL_STATE.lock().unwrap();

        let render: bool = process::update::process(&mut state.bctx, msg);

        return render;
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let mut state = GLOBAL_STATE.lock().unwrap();

        let page = state.bctx.page.clone();

        if page == Page::Home {
            view::home::home_view(&mut state.bctx)
        } else if page == Page::Collections {
            view::collections::collections_view(&mut state.bctx)
        } else {
            view::home::home_view(&mut state.bctx)
        }
    }
}

fn send_request(request: Request) {
    #[derive(Debug, Serialize, Deserialize)]
    struct Payload {
        url: String,
        method: Method,
        body: String,
        headers: Vec<Vec<String>>,
        index: usize,
    }

    let payload = Payload {
        url: parse_url(request.url, request.params),
        method: request.method,
        body: request.body,
        headers: request.headers,
        index: request.request_index,
    };

    // bolt_log(&format!("{:?}", payload));

    wasm_bindgen_futures::spawn_local(async move {
        let _resp: String = tauri::invoke("send_request", &payload).await.unwrap();
    });
}

pub fn receive_response(data: &str) {
    let mut state = GLOBAL_STATE.lock().unwrap();
    let bctx = &mut state.bctx;

    // bolt_log("received a response");

    let mut response: Response = serde_json::from_str(data).unwrap();

    // bolt_log(&format!("{:?}", response));

    if response.response_type == ResponseType::JSON {
        response.body = format_json(&response.body);
        response.body = highlight_body(&response.body);
    }

    if bctx.page == Page::Home {
        let current = response.request_index;
        state.bctx.main_col.requests[current].response = response;
    } else {
        let current = &bctx.col_current;
        bctx.collections[current[0]].requests[current[1]].response = response;
    }

    let link = state.bctx.link.as_ref().unwrap();

    link.send_message(Msg::Update);
}

fn main() {
    wasm_bindgen_futures::spawn_local(async move {
        let mut events = tauri_sys::event::listen::<String>("receive_response")
            .await
            .expect("could not create response listener");

        while let Some(event) = events.next().await {
            receive_response(&event.payload);
        }
    });

    yew::Renderer::<BoltApp>::new().render();
}

// pub fn resizable(bctx: &mut BoltContext) {
//     bctx.resized = true;

//     _bolt_log("did resizable");
//     let window = web_sys::window().unwrap();
//     let document = web_sys::Window::document(&window).unwrap();

//     // let resizer = web_sys::Document::get_element_by_id(&doc, "resizer")
//     //     .unwrap()
//     //     .dyn_into::<web_sys::HtmlElement>()
//     //     .unwrap();

//     // let sidebar = web_sys::Document::get_element_by_id(&doc, "sidebars")
//     //     .unwrap()
//     //     .dyn_into::<web_sys::HtmlElement>()
//     //     .unwrap();

//     let resizer = document.query_selector(".resizer").unwrap().unwrap();
//     let sidebar = document
//         .query_selector(".sidebars")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<HtmlElement>()
//         .unwrap();

//     let sec_sidebar = sidebar.clone();

//     let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
//         let size = format!("{}px", event.client_x());
//         sidebar.style().set_property("flex-basis", &size).unwrap();
//     }) as Box<dyn FnMut(MouseEvent)>);

//     resizer
//         .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
//         .unwrap();

//     closure.forget();
//     sec_sidebar
//         .style()
//         .set_property("flex-basis", "325px")
//         .unwrap();
// }
