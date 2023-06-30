use super::sections::{home::Home, settings::lib::Settings};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable, Copy)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    SettingsSubDirectories,
}

impl Route {
    pub fn get_list() -> Vec<Route> {
        vec![Route::Home, Route::SettingsRoot]
    }

    pub fn has_submenu(self) -> bool {
        match self {
            Route::SettingsSubDirectories => true,
            _ => false,
        }
    }

    pub fn string(self) -> String {
        match self {
            Route::Home => "Home".to_owned(),
            Route::SettingsSubDirectories => "SettingsSubDirectories".to_owned(),
            Route::SettingsRoot => "Settings".to_owned(),
        }
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html!(<Home />),
        Route::SettingsSubDirectories | Route::SettingsRoot => {
            html!(<Switch<SettingsRoute> render={settings_switch} />)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Routable, Copy)]
pub enum SettingsRoute {
    #[at("/settings")]
    Settings,
    #[at("/settings/account")]
    Account,
    #[at("/settings/notifications")]
    Notificaitons,
    #[at("/settings/preferences")]
    Preferences,
}

impl SettingsRoute {
    pub fn get_list() -> Vec<SettingsRoute> {
        vec![
            SettingsRoute::Account,
            SettingsRoute::Notificaitons,
            SettingsRoute::Preferences,
        ]
    }

    pub fn string(self) -> String {
        match self {
            SettingsRoute::Settings => "Settings".to_owned(),
            SettingsRoute::Account => "Account".to_owned(),
            SettingsRoute::Notificaitons => "Notifications".to_owned(),
            SettingsRoute::Preferences => "Preferences".to_owned(),
        }
    }
}

fn settings_switch(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Settings => html!(<Settings page={SettingsRoute::Settings}/>),
        SettingsRoute::Account => html!(<Settings page={SettingsRoute::Account}/>),
        SettingsRoute::Notificaitons => html!(<Settings page={SettingsRoute::Notificaitons}/>),
        SettingsRoute::Preferences => html!(<Settings page={SettingsRoute::Preferences}/>),
    }
}

#[function_component(Routing)]
pub fn routing() -> Html {
    html! {
        <div class="routing">
        <Switch<Route> render={switch}/>
        </div>
    }
}
