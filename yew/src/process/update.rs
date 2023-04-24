// use crate::save_state;
use crate::send_request;
use crate::utils::*;
use crate::BoltContext;
use crate::Collection;
use crate::Msg;
use crate::Page;
use crate::Request;

pub fn process(bctx: &mut BoltContext, msg: Msg) -> bool {
    let should_render = match msg {
        Msg::Nothing => false,

        Msg::SelectedMethod(meth) => {
            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].method = meth;
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]].method = meth;
            }

            true
        }

        Msg::SendPressed => {
            if bctx.page == Page::Home {
                let req = &mut bctx.main_col.requests[bctx.main_current];
                send_request(req);
            } else {
                let current = &bctx.col_current;
                let req = &mut bctx.collections[current[0]].requests[current[1]];
                send_request(req);
            }

            true
        }

        Msg::HelpPressed => {
            open_link("https://github.com/hiro-codes/bolt".to_string());

            true
        }

        Msg::ReqBodyPressed => {
            if bctx.page == Page::Home {
                let req = &mut bctx.main_col.requests[bctx.main_current];

                req.req_tab = 1;
            } else {
                let current = &bctx.col_current;
                let req = &mut bctx.collections[current[0]].requests[current[1]];
                req.req_tab = 1;
            }

            true
        }

        Msg::ReqHeadersPressed => {
            if bctx.page == Page::Home {
                let req = &mut bctx.main_col.requests[bctx.main_current];

                req.req_tab = 3;
            } else {
                let current = &bctx.col_current;
                let req = &mut bctx.collections[current[0]].requests[current[1]];
                req.req_tab = 3;
            }

            true
        }

        Msg::ReqParamsPressed => {
            if bctx.page == Page::Home {
                let req = &mut bctx.main_col.requests[bctx.main_current];

                req.req_tab = 2;
            } else {
                let current = &bctx.col_current;
                let req = &mut bctx.collections[current[0]].requests[current[1]];
                req.req_tab = 2;
            }

            true
        }

        Msg::RespBodyPressed => {
            if bctx.page == Page::Home {
                let mut req = &mut bctx.main_col.requests[bctx.main_current];

                req.resp_tab = 1;
            } else {
                let current = &bctx.col_current;
                let req = &mut bctx.collections[current[0]].requests[current[1]];
                req.resp_tab = 1;
            }

            true
        }

        Msg::RespHeadersPressed => {
            if bctx.page == Page::Home {
                let req = &mut bctx.main_col.requests[bctx.main_current];

                req.resp_tab = 2;
            } else {
                let current = &bctx.col_current;
                let req = &mut bctx.collections[current[0]].requests[current[1]];
                req.resp_tab = 2;
            }

            true
        }

        Msg::ReceivedResponse => true,

        Msg::AddHeader => {
            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current]
                    .headers
                    .push(vec!["".to_string(), "".to_string()]);
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]]
                    .headers
                    .push(vec!["".to_string(), "".to_string()]);
            }
            true
        }

        Msg::RemoveHeader(index) => {
            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].headers.remove(index);
            } else {
                let current = &bctx.col_current;

                bctx.collections[current[0]].requests[current[1]]
                    .headers
                    .remove(index);
            }

            true
        }

        Msg::AddParam => {
            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current]
                    .params
                    .push(vec!["".to_string(), "".to_string()]);
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]]
                    .params
                    .push(vec!["".to_string(), "".to_string()]);
            }
            true
        }

        Msg::AddCollection => {
            let mut new_collection = Collection::new();

            new_collection.name = new_collection.name + &(bctx.collections.len() + 1).to_string();
            bctx.collections.push(new_collection);

            true
        }

        Msg::RemoveCollection(index) => {
            bctx.collections.remove(index);

            bctx.col_current = vec![0, 0];

            true
        }

        Msg::RemoveParam(index) => {
            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].params.remove(index);
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]]
                    .params
                    .remove(index);
            }

            true
        }

        Msg::MethodChanged => {
            let method = get_method();

            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].method = method;
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]].method = method;
            }

            true
        }

        Msg::UrlChanged => {
            let url = get_url();

            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].url = url.clone();
                bctx.main_col.requests[current].name = url;
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]].url = url.clone();
                bctx.collections[current[0]].requests[current[1]].name = url;
            }

            true
        }

        Msg::BodyChanged => {
            let body = get_body();

            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].body = body;
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]].body = body;
            }

            true
        }

        Msg::HeaderChanged(index) => {
            let header = get_header(index);

            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].headers[index] = header;
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]].headers[index] = header;
            }

            true
        }

        Msg::ParamChanged(index) => {
            let param = get_param(index);

            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].params[index] = param;
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]].params[index] = param;
            }

            true
        }

        Msg::AddRequest => {
            let mut new_request = Request::new();
            new_request.name = new_request.name + &(bctx.main_col.requests.len() + 1).to_string();

            bctx.main_col.requests.push(new_request);

            true
        }

        Msg::AddToCollection(index) => {
            let collection = &mut bctx.collections[index];

            let mut new_request = Request::new();
            new_request.name = new_request.name + &(collection.requests.len() + 1).to_string();

            collection.requests.push(new_request);

            true
        }

        Msg::ToggleCollapsed(index) => {
            let collection = &mut bctx.collections[index];

            collection.collapsed = !collection.collapsed;

            true
        }

        Msg::RemoveRequest(index) => {
            bctx.main_col.requests.remove(index);
            if !bctx.main_col.requests.is_empty()
                && bctx.main_current > bctx.main_col.requests.len() - 1
            {
                bctx.main_current = bctx.main_col.requests.len() - 1;
            }

            true
        }

        Msg::RemoveFromCollection(col_index, req_index) => {
            bctx.collections[col_index].requests.remove(req_index);
            bctx.col_current = vec![0, 0];

            true
        }

        Msg::SelectRequest(index) => {
            let mut new_index = index;

            if bctx.main_col.requests.len() == 0 {
                bctx.main_current = new_index;

                // bctx.main_col.requests[new_index].response.request_index = new_index;
            } else {
                if index >= bctx.main_col.requests.len() {
                    new_index = bctx.main_col.requests.len() - 1;
                    bctx.main_current = new_index;

                    bctx.main_col.requests[new_index].response.request_index = new_index;
                } else {
                    bctx.main_current = new_index;

                    bctx.main_col.requests[new_index].response.request_index = new_index;
                }
            }

            true
        }

        Msg::SelectFromCollection(col_index, req_index) => {
            bctx.col_current = vec![col_index, req_index];

            bctx.collections[col_index].requests[req_index]
                .response
                .request_index = req_index;

            true
        }

        Msg::Update => true,

        Msg::SwitchPage(page) => {
            bctx.page = page;

            true
        }
    };

    should_render
}
