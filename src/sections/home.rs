use super::components::task::Task;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
            <Task id=1/>
            <Task id=2/>
            <Task id=3/>
            <Task id=4/>
        </div>
    }
}
