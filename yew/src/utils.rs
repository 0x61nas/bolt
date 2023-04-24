use crate::BoltContext;
use crate::Method;
use crate::Msg;
use crate::Request;
use crate::SaveState;
use crate::GLOBAL_STATE;
use serde::{Deserialize, Serialize};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, MouseEvent};

use syntect::highlighting::ThemeSet;
use syntect::highlighting::{Color, Theme};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[allow(dead_code)]
static BACKEND: &str = "http://0.0.0.0:3344/";

pub fn _bolt_log(_log: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(_log));
}

pub fn invoke_send(request: &mut Request) {
    #[derive(Debug, Serialize, Clone, Deserialize)]
    pub struct SendPayload {
        url: String,
        method: Method,
        body: String,
        headers: Vec<Vec<String>>,
        index: usize,
    }

    let payload = SendPayload {
        url: parse_url(request.url.clone(), request.params.clone()),
        method: request.method,
        body: request.body.clone(),
        headers: request.headers.clone(),
        index: request.response.request_index,
    };

    let _payload = payload.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let payload = serde_json::to_string(&_payload).unwrap();

        let client = reqwest::Client::new();

        let res = client
            .post(BACKEND.to_string() + "send_request")
            .body(payload)
            .send()
            .await
            .expect("send request failed");

        let resp = res.text().await.unwrap();
        crate::receive_response(&resp);
    });
}

pub fn save_state(bctx: &mut BoltContext) {
    let save_state = SaveState {
        page: bctx.page.clone(),
        main_current: bctx.main_current.clone(),
        col_current: bctx.col_current.clone(),

        main_col: bctx.main_col.clone(),
        collections: bctx.collections.clone(),
    };

    let _save = serde_json::to_string(&save_state).unwrap();
    let _save2 = _save.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let client = reqwest::Client::new();

        let res = client
            .post(BACKEND.to_string() + "save_state")
            .body(_save2)
            .send()
            .await
            .expect("save state failed");

        let _resp = res.text().await.unwrap();
    });
}

fn set_save_state(state: String) {
    let new_state: SaveState = serde_json::from_str(&state).unwrap();

    let mut global_state = GLOBAL_STATE.lock().unwrap();

    global_state.bctx.main_col = new_state.main_col;
    global_state.bctx.collections = new_state.collections;

    global_state.bctx.col_current = new_state.col_current;
    global_state.bctx.main_current = new_state.main_current;

    global_state.bctx.page = new_state.page;

    let link = global_state.bctx.link.as_ref().unwrap();
    link.send_message(Msg::Update);
}

pub fn restore_state() {
    wasm_bindgen_futures::spawn_local(async move {
        let client = reqwest::Client::new();

        let res = client
            .post(BACKEND.to_string() + "restore_state")
            .send()
            .await
            .expect("restore state failed");

        let resp = res.text().await.unwrap();
        // _bolt_log(&text);

        set_save_state(resp);
    });
}

pub fn open_link(link: String) {
    let _link = link.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let client = reqwest::Client::new();

        let res = client
            .post(BACKEND.to_string() + "open_link")
            .body(_link)
            .send()
            .await
            .expect("open_link failed");

        let _resp = res.text().await.unwrap();
    });
}

pub fn bolt_panic(log: &str) {
    #[derive(Serialize, Deserialize)]
    struct Payload<'a> {
        log: &'a str,
    }

    let log = log.to_string();

    panic!("{}", log);
}

pub fn _set_html(id: &str, content: String) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, id).unwrap();

    div.set_inner_html(&content);
}

pub fn _set_focus(id: &str) {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, id).unwrap();

    let div = div.dyn_into::<web_sys::HtmlElement>().unwrap();

    div.focus().unwrap();
}

pub fn get_method() -> Method {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "methodselect").unwrap();

    let select = div.dyn_into::<web_sys::HtmlSelectElement>().unwrap();

    let value = select.value();

    match value.as_str() {
        "get" => Method::GET,
        "post" => Method::POST,
        "put" => Method::PUT,
        "delete" => Method::DELETE,
        "head" => Method::HEAD,
        "patch" => Method::PATCH,
        "options" => Method::OPTIONS,
        "connect" => Method::CONNECT,

        _ => {
            bolt_panic("invalid method");

            Method::GET
        }
    }
}

pub fn get_url() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "urlinput").unwrap();

    div.dyn_into::<web_sys::HtmlInputElement>().unwrap().value()
}

pub fn get_body() -> String {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();
    let div = web_sys::Document::get_element_by_id(&doc, "reqbody").unwrap();

    div.dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .value()
}

pub fn get_header(index: usize) -> Vec<String> {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();

    let key =
        web_sys::Document::get_element_by_id(&doc, &("headerkey".to_string() + &index.to_string()))
            .unwrap();
    let value = web_sys::Document::get_element_by_id(
        &doc,
        &("headervalue".to_string() + &index.to_string()),
    )
    .unwrap();

    let key = key.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let value = value.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    vec![key.value(), value.value()]
}

pub fn get_param(index: usize) -> Vec<String> {
    let window = web_sys::window().unwrap();
    let doc = web_sys::Window::document(&window).unwrap();

    let key =
        web_sys::Document::get_element_by_id(&doc, &("paramkey".to_string() + &index.to_string()))
            .unwrap();
    let value = web_sys::Document::get_element_by_id(
        &doc,
        &("paramvalue".to_string() + &index.to_string()),
    )
    .unwrap();

    let key = key.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let value = value.dyn_into::<web_sys::HtmlInputElement>().unwrap();

    vec![key.value(), value.value()]
}

// HACK: disables selecting text
pub fn disable_text_selection() {
    if let Some(document) = web_sys::window().and_then(|win| win.document()) {
        if let Some(body) = document.body() {
            let listener = Closure::wrap(Box::new(move |event: MouseEvent| {
                event.prevent_default();
            }) as Box<dyn FnMut(_)>);
            let _ = EventTarget::from(body)
                .add_event_listener_with_callback("selectstart", listener.as_ref().unchecked_ref());
            listener.forget();
        }
    }
}

pub fn format_json(data: &str) -> String {
    let value: serde_json::Value = serde_json::from_str(data).unwrap();

    serde_json::to_string_pretty(&value).unwrap()
}

fn create_custom_theme() -> Theme {
    let mut theme = ThemeSet::load_defaults().themes["Solarized (dark)"].clone();

    // Change the background color
    theme.settings.background = Some(Color {
        r: 3,
        g: 7,
        b: 13,
        a: 1,
    });

    theme
}

pub fn highlight_body(body: &str) -> String {
    // Add syntax highlighting
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme = create_custom_theme();
    let syntax = syntax_set.find_syntax_by_extension("json").unwrap();

    highlighted_html_for_string(body, &syntax_set, syntax, &theme).unwrap()
}

pub fn parse_url(url: String, params: Vec<Vec<String>>) -> String {
    let mut new_url = url;

    if !params.is_empty() && !params[0][0].is_empty() {
        new_url.push('?');
    }

    for (i, param) in params.iter().enumerate() {
        if param[0].is_empty() || param[1].is_empty() {
            continue;
        }

        new_url.push_str(&param[0]);
        new_url.push('=');
        new_url.push_str(&param[1]);

        if i != params.len() - 1 {
            new_url.push('&');
        }
    }

    // bolt_log(&format!("url is: {new_url}"));
    new_url
}
