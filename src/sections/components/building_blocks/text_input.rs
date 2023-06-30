use yew::{function_component, html, prelude::Html, Properties};

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub placeholder: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    html! {
        <input type="text" placeholder={props.placeholder.clone()} />
    }
}
