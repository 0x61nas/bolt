use crate::BoltContext;
use crate::Msg;
use yew::{html, Html};

pub fn render_params(
    bctx: &mut BoltContext,
    index: usize,
    length: usize,
    key: &String,
    value: &String,
) -> Html {
     let link = bctx.link.as_ref().unwrap();

    html! {
        <tr>
            <td><input id={"paramkey".to_string() + &index.to_string()} type="text" class="tableinput" value={key.to_string()} onchange={link.callback(move |_| Msg::ParamChanged(index))}/></td>
            <td class="tableline">
                <input id={"paramvalue".to_string() + &index.to_string()} type="text" class="tableinput" value={value.to_string()} onchange={link.callback(move |_| Msg::ParamChanged(index))}/>
                if index == length - 1 {
                    <div class="pointer" onclick={link.callback(|_| Msg::AddParam)}>
                        <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                    </div>
                }else {
                    <div class="pointer" onclick={link.callback(move |_| Msg::RemoveParam(index))}>
                        <svg viewBox="0 0 1024 1024" fill="currentColor" height="1em" width="1em"> <path d="M864 256H736v-80c0-35.3-28.7-64-64-64H352c-35.3 0-64 28.7-64 64v80H160c-17.7 0-32 14.3-32 32v32c0 4.4 3.6 8 8 8h60.4l24.7 523c1.6 34.1 29.8 61 63.9 61h454c34.2 0 62.3-26.8 63.9-61l24.7-523H888c4.4 0 8-3.6 8-8v-32c0-17.7-14.3-32-32-32zm-200 0H360v-72h304v72z" /> </svg>
                    </div>
                }
            </td>
        </tr>
    }
}
