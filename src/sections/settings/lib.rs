use super::super::super::router::SettingsRoute;
use super::account::Account;
use super::notifications::Notifications;
use super::preferences::Preferences;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub page: SettingsRoute,
}

#[function_component(Settings)]
pub fn settings(props: &Props) -> Html {
    let component = match props.page {
        SettingsRoute::Account => html!(<Account />),
        SettingsRoute::Settings => html!(),
        SettingsRoute::Notificaitons => html!(<Notifications />),
        SettingsRoute::Preferences => html!(<Preferences />),
    };
    html! {
        <div>
            <h1>{"Settings"}</h1>
            { component }
        </div>
    }
}
