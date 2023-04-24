use crate::BoltContext;
use crate::view;
use yew::{html, Html};

pub fn home_view(bctx: &mut BoltContext) -> Html {
    html! {
       <body>
            {view::navbar::get_navbar(bctx)}

            <div class="main">
                <div class="sidebars">
                    {view::sidebar1::sidebar(bctx, 0)}
                    {view::sidebar2::sidebar_requests(bctx)}
                </div>
                
                <div class="resizer"></div>
        
                <div class="content">
                    {view::request::request(bctx)}

                    <div class="resizer2"></div>
        
                    {view::response::response(bctx)}
                </div>
            </div>

       </body>
    }
}
