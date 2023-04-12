use crate::send_request;
use crate::utils::*;
use crate::BoltContext;
use crate::Collection;
use crate::Msg;
use crate::Page;
use crate::Request;

pub fn process(bctx: &mut BoltContext, msg: Msg) -> bool {
    match msg {
        
        Msg::Nothing => {false}
        
        Msg::SelectedMethod(meth) => {
            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].method = meth;
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]].method = meth;
            }

            return true;
        }

        Msg::SendPressed => {
            if bctx.page == Page::Home {
                let req = bctx.main_col.requests[bctx.main_current].clone();
                send_request(req);
            } else {
                let current = &bctx.col_current;
                let req = bctx.collections[current[0]].requests[current[1]].clone();
                send_request(req);
            }

            return true;
        }

        Msg::HelpPressed => {
            open_link("https://github.com/hiro-codes/bolt".to_string());

            return true;
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

            return true;
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

            return true;
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

            return true;
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

            return true;
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

            return true;
        }

        Msg::ReceivedResponse => {
            return true;
        }

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
            return true;
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

            return true;
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
            return true;
        }

        Msg::AddCollection => {
            let mut new_collection = Collection::new();

            new_collection.name = new_collection.name + &(bctx.collections.len() + 1).to_string();
            bctx.collections.push(new_collection);

            return true;
        }

        Msg::RemoveCollection(index) => {
            bctx.collections.remove(index);

            bctx.col_current = vec![0, 0];

            return true;
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

            return true;
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

            return true;
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

            return true;
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

            return true;
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

            return true;
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

            return true;
        }

        Msg::AddRequest => {
            let mut new_request = Request::new();
            new_request.name = new_request.name + &(bctx.main_col.requests.len() + 1).to_string();

            bctx.main_col.requests.push(new_request);

            return true;
        }

        Msg::AddToCollection(index) => {
            let collection = &mut bctx.collections[index];

            let mut new_request = Request::new();
            new_request.name = new_request.name + &(collection.requests.len() + 1).to_string();

            collection.requests.push(new_request);

            return true;
        }

        Msg::ToggleCollapsed(index) => {
            let collection = &mut bctx.collections[index];

            collection.collapsed = !collection.collapsed;

            return true;
        }

        Msg::RemoveRequest(index) => {
            bctx.main_col.requests.remove(index);
            if bctx.main_col.requests.len() > 0
                && bctx.main_current > bctx.main_col.requests.len() - 1
            {
                bctx.main_current = bctx.main_col.requests.len() - 1;
            }

            return true;
        }

        Msg::RemoveFromCollection(col_index, req_index) => {
            bctx.collections[col_index].requests.remove(req_index);
            bctx.col_current = vec![0, 0];

            return true;
        }

        Msg::SelectRequest(index) => {
            bctx.main_current = index;

            bctx.main_col.requests[index].response.request_index = index;

            return true;
        }

        Msg::SelectFromCollection(col_index, req_index) => {
            bctx.col_current = vec![col_index, req_index];

            bctx.collections[col_index].requests[req_index].response.request_index = req_index;

            return true;
        }

        Msg::Update => {
            return true;
        }

        Msg::SwitchPage(page) => {
            bctx.page = page;

            return true;
        }
    }
}
