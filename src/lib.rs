mod router;
mod sections;

use crate::sections::components::sidebar::Sidebar;
use router::Routing;
use stylist::style;
use yew::prelude::*;
use yew_router::BrowserRouter;

#[function_component(App)]
pub fn view() -> Html {
    // lookup issue in 100vh on mobile devices and find a better solution
    let stylesheet = style!(
        r#"
        .sidebar {
            background-color: grey;
            float: left;
            width: auto;
            height: 100vh;
            padding: 0px;
            margin: 0px;
            border: 0px;
        }
        .routing {
            background-color: cyan;
            float: right;
            width: 85%;
            height: 100vh;
            padding: 0px;
            border: 0px;
            margin: 0px;
        }
        "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
        <BrowserRouter>
            <Sidebar />
            <Routing />
        </BrowserRouter>
        </div>
    }
}
