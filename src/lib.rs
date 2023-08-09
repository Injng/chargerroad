use yew::prelude::*;
use stylist::yew::styled_component;

mod components;

use components::road::Road;

#[styled_component]
pub fn App() -> Html {
    html! {
        <div>
            <Road />
        </div>
    }
}
