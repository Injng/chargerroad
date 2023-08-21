use yew::prelude::*;
use stylist::{yew::styled_component, Style};

mod components;

use components::road::Road;
use components::courses::Courses;

const STYLE_FILE: &str = include_str!("styles/style.css");


#[styled_component]
pub fn App() -> Html {
    let gen_style = Style::new(STYLE_FILE).unwrap();
    html! {
        <div class={gen_style}>
            <div class="main">
                <div class="road">   
                    <Road />
                </div>
                <div class="courses">
                    <Courses />
                </div>
            </div>
        </div>
    }
}

