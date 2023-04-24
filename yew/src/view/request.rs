// use web_sys::InputEvent;
use yew::KeyboardEvent;
use crate::view;
use crate::BoltContext;
use crate::Msg;
use crate::Page;
use crate::Request;
use yew::{html, Html};
use crate::helpers::enums::{
    RequestTabs::{self, Body, Params, Headers},
    HttpMethod,
};

pub fn request(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    let can_display = (bctx.page == Page::Collections
        && !bctx.collections.is_empty()
        && !bctx.collections[bctx.col_current[0]].requests.is_empty())
        || (bctx.page == Page::Home && !bctx.main_col.requests.is_empty());

    let mut request = Request::new();

    if can_display {
        request = if bctx.page == Page::Home {
            bctx.main_col.requests[bctx.main_current].clone()
        } else if bctx.page == Page::Collections {
            bctx.collections[bctx.col_current[0]].requests[bctx.col_current[1]].clone()
        } else {
            Request::new()
        };
    }

    let selected_method = request.method.to_string();

    html! {
        <div class="req">
        if can_display {
            <div class="requestbar">
                <div class="">
                    <select id="methodselect" class="methodselect pointer" onchange={link.callback(|_| Msg::MethodChanged)}>
                        { for (0..HttpMethod::count()).map(|index| {
                            let current_method_option: HttpMethod = HttpMethod::from(index);
                            let value = current_method_option.to_string().to_lowercase();
                            html! {
                                <option value={value.clone()} selected={is_selected(&selected_method, &value)}>{current_method_option}</option>
                            }
                        })}
                    </select>
                </div>

                <input id="urlinput" class="urlinput" type="text" value={request.url.clone()} placeholder="http://" onkeydown={link.callback(|e: KeyboardEvent| { if e.key() == "Enter" { Msg::SendPressed } else { Msg::Nothing } })}  oninput={link.callback(|_|{ Msg::UrlChanged })} />

                <button class="sendbtn pointer" type="button" onclick={link.callback(|_| Msg::SendPressed)}>{"Send"}</button>
            </div>

            <div class="reqtabs">
                <div id="req_body_tab" class={if is_tab_selected(&request.req_tab, Body) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::ReqBodyPressed)}>{"Body"}</div>
                <div id="req_params_tab" class={if is_tab_selected(&request.req_tab, Params) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::ReqParamsPressed)}>{"Params"}</div>
                <div id="req_headers_tab" class={if is_tab_selected(&request.req_tab, Headers) {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::ReqHeadersPressed)}>{"Headers"}</div>
            </div>

            <div class="tabcontent">
                if is_tab_selected(&request.req_tab, Body) {
                    <textarea id="reqbody" class="reqbody" value={request.body.clone()} placeholder="Request body" onchange={link.callback(|_| Msg::BodyChanged)}>

                    </textarea>
                } else if is_tab_selected(&request.req_tab, Params) {
                    <div class="reqheaders">
                        <table>
                            <tr>
                                <th>{"Key"}</th>
                                <th>{"Value"}</th>
                            </tr>
                            { for request.params.iter().enumerate().map(|(index, header)| view::param::render_params(bctx, index, request.params.len(), &header[0], &header[1])) }
                        </table>
                    </div>

                } else if is_tab_selected(&request.req_tab, Headers) {
                    <div class="reqheaders">
                        <table>
                            <tr>
                                <th>{"Header"}</th>
                                <th>{"Value"}</th>
                            </tr>
                            { for request.headers.iter().enumerate().map(|(index, header)| view::header::render_reqheader(bctx, index, request.headers.len(), &header[0], &header[1])) }
                        </table>
                    </div>
                }
            </div>
        }
        </div>

    }
}

fn is_selected(method: &str, option_value: &str) -> bool {
    method.to_lowercase() == option_value.to_lowercase()
}

fn is_tab_selected(request_tab: &u8, tab: RequestTabs) -> bool {
    *request_tab == u8::from(tab)
}