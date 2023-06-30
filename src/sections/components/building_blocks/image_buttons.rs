use stylist::style;
use yew::prelude::*;

#[function_component(ImageButton)]
pub fn image_button() -> Html {
    let stylesheet = style!(
        r#"
        img.resize {
            width: 20px;
            height: auto;
        }
        "#
    )
    .unwrap();
    html! {
        <div class={stylesheet}>
            <img class="resize" src="public/icons/sidebar_button.jpg" alt="sidebar button"/>
        </div>
    }
}
