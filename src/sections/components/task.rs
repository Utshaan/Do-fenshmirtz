use super::building_blocks::text_input::TextInput;
use yew::{function_component, html, prelude::Html, Properties};

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub id: usize,
}

#[function_component(Task)]
pub fn task(props: &Props) -> Html {
    html! {
        <div>
            <a>{format!("{})", props.id)}</a>
            <TextInput placeholder={format!("Task {}...", props.id)}/>
        </div>
    }
}
