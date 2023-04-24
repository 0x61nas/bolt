use crate::BoltContext;
use crate::Collection;
use crate::Msg;
use crate::Request;
use yew::{html, Html};

pub fn sidebar_requests(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div class="sidebar2">
            <div>
                <div class="pointer" onclick={link.callback(|_| Msg::AddRequest)}>
                    <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                </div>
            </div>

            { for bctx.main_col.requests.iter().enumerate().map(|(index, req)| render_request(&mut bctx.clone(), bctx.main_current, index, req))}

        </div>
    }
}

pub fn sidebar_col(bctx: &mut BoltContext) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div class="sidebar2">
            <div>
                <div class="pointer" onclick={link.callback(|_| Msg::AddCollection)}>
                    <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                </div>
            </div>

            { for bctx.collections.iter().enumerate().map(|(index, col)| render_collection(&mut bctx.clone(), index, bctx.col_current.clone(), col))}

        </div>
    }
}

fn render_collection(
    bctx: &mut BoltContext,
    index: usize,
    current: Vec<usize>,
    col: &Collection,
) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <>
        <div id={"request".to_string() + &index.to_string()} class="sidebar2item">

            if col.collapsed {
                <div onclick={link.callback(move |_| Msg::ToggleCollapsed(index))} class="col-arrow pointer">{">"}</div>
            } else {
                <div onclick={link.callback(move |_| Msg::ToggleCollapsed(index))} class="col-arrow pointer">{"⌄"}</div>
            }

            <div>{col.name.clone()}</div>
            
            <div class="col-icons">        
            <div class="pointer add-col" onclick={link.callback(move |_| Msg::AddToCollection(index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="15px" width="15px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
            </div>

            <div class="pointer bin-col" onclick={link.callback(move |_| Msg::RemoveCollection(index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="15px" width="15px"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
            </div>
            </div>
        </div>
        if !col.collapsed {
            { for col.requests.iter().enumerate().map(|(req_index, req)| render_col_request(bctx, req_index, index, current.clone(), req))}
        }
        
        </>
    }
}

fn render_request(bctx: &mut BoltContext, current: usize, index: usize, req: &Request) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div onclick={link.callback(move |_| Msg::SelectRequest(index))} id={"request".to_string() + &index.to_string()} class={if index == current { "pointer sidebar2item sidebar2item-selected" } else { "pointer sidebar2item" }} >
            <div>{req.name.clone()}</div>
            <div class="pointer bin-req" onclick={link.callback(move |_| Msg::RemoveRequest(index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
            </div>
        </div>
    }
}

fn render_col_request(
    bctx: &mut BoltContext,
    req_index: usize,
    col_index: usize,
    current: Vec<usize>,
    req: &Request,
) -> Html {
    let link = bctx.link.as_ref().unwrap();

    html! {
        <div id={"request".to_string() + &req_index.to_string()} class={if col_index == current[0] && req_index == current[1] { "sidebar2item-child sidebar2item-selected" } else { "sidebar2item-child" }} >
            <div class="pointer" onclick={link.callback(move |_| Msg::SelectFromCollection(col_index, req_index))}>{req.name.clone()}</div>
            <div class="pointer bin-req" onclick={link.callback(move |_| Msg::RemoveFromCollection(col_index, req_index))}>
                <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
            </div>
        </div>
    }
}
