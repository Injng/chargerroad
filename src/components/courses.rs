use yew::prelude::*;
use yew::Properties;
use stylist::{yew::styled_component, Style};  

#[derive(Properties, PartialEq)]
struct Props {
    name: String,
    prct: u8,
}

const STYLE_FILE: &str = include_str!("../styles/courses.css");

#[styled_component]
fn CourseList(props: &Props) -> Html {
    let gen_style = Style::new(STYLE_FILE).unwrap();
    let show_state = use_state(|| "none");
    let show = *show_state;
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
        .progress-bar {
            width: 100%;
            background-color: #d1d1d1;
            border-weight: 2px;
            height: 5px;
        }

        .bar {
            width: ${prct}%;
            background-color: green;
            height: 5px;
        }
        
        .hidden-content {
            display: ${show};
        }", prct=props.prct, show=show};


    html! {
        <div class={classes!(gen_style, style)}>
            <button class="wrap-list" onclick={onclick}>
                <span class="list-name">{props.name.clone()}</span>
                <div class="progress-bar">
                    <div class="bar"></div>
                </div>
            </button>
            
            <div class="hidden-content">
                <p>{"This is a test"}</p>
            </div>
        </div>  
    } 
}

#[function_component]
pub fn Courses() -> Html {
    html! {
        <CourseList name="Test" prct=10 />
    }
}
