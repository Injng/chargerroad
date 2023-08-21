use yew::prelude::*;
use yew::Properties;
use stylist::{yew::styled_component, Style};

#[derive(Properties, PartialEq)]
struct Props {
    title: String,
    content: String,
}

const STYLE_FILE: &str = include_str!("../styles/road.css");

#[styled_component]
fn Collapsible(props: &Props) -> Html {
    let show_state = use_state(|| "none");
    let show = *show_state;
    let gen_style = Style::new(STYLE_FILE).unwrap();
    let onclick = Callback::from({
        let show_state = show_state.clone();
        move |_| {
            if *show_state == "none" {
                show_state.set("block");
            } else {
                show_state.set("none");
            }
        }
    });
    
    let style = css! {"
        .collapsible-content {
            display: ${show};
            background: #d1d1d1;
            border-bottom: 1px solid #d1d1d1;
            border-bottom-left-radius: 7px;
            border-bottom-right-radius: 7px;
            padding: .5rem 1rem;
            transition: display 1s ease-out;
        }", show=show};


    html! {
        <div class={classes!(gen_style, style)}>
            <div class="wrap-collapsible">
                <button class="button-collapse" type="button" onclick={onclick}>
                    <span class="button-title">{props.title.clone()}</span>
                    <span class="button-units">{"Units: test"}</span>
                </button>
                <div class="collapsible-content">
                    <div class="content-inner">
                        <p>
                            {props.content.clone()}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn Road() -> Html {
    html! {
        <div>
            <div class="road">
                <Collapsible title="Prior Credit" content="test" />
                <Collapsible title="Freshman 9" content="test" />
                <Collapsible title="Summer 9" content="test" />
                <Collapsible title="Sophomore 10" content="test" />
                <Collapsible title="Summer 10" content="test" />
                <Collapsible title="Junior 11" content="test" />
                <Collapsible title="Summer 11" content="test" />
                <Collapsible title="Senior 12" content="test" />
                <Collapsible title="Summer 12" content="test" />
            </div>
        </div>
    }
}

