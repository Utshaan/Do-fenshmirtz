use super::building_blocks::image_buttons::ImageButton;
use crate::router::{Route, SettingsRoute};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone)]
pub struct Menu {
    pub display: String,
    pub route: Route,
    pub submenus: Option<Vec<Menu>>,
}

impl Menu {
    pub fn new(display: String, route: Route) -> Menu {
        Menu {
            display,
            route,
            submenus: None,
        }
    }
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let menus = Route::get_list()
        .iter()
        .map(|route| {
            if route.has_submenu() {
                let menu = Menu::new(route.string(), *route);
                html! {
                    <div><Link<Route> to={menu.route}>{menu.display}</Link<Route>></div>
                }
            } else {
                let menu = Menu::new(route.string(), *route);
                html! {
                    <div><Link<Route> to={menu.route}>{menu.display}</Link<Route>></div>
                }
            }
        })
        .collect::<Html>();

    html! {
        <div class="sidebar">
            <ImageButton />
            { menus }
            <div><Link<SettingsRoute> to={SettingsRoute::Account}>{"    Account"}</Link<SettingsRoute>></div>
            <div><Link<SettingsRoute> to={SettingsRoute::Notificaitons}>{"  Notifications"}</Link<SettingsRoute>></div>
            <div><Link<SettingsRoute> to={SettingsRoute::Preferences}>{"    Preferences"}</Link<SettingsRoute>></div>
        </div>
    }
}
