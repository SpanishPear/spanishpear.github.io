use crate::Route;
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct ButtonProps {
    pub label: String,
    pub to: Route,
    pub classes: Option<String>,
}

#[function_component(LinkButton)]
pub fn link_button(props: &ButtonProps) -> Html {
    let button_classes = vec![
        "focus:outline-none",
        "text-white",
        "bg-purple-700",
        "hover:bg-purple-800",
        "focus:ring-4",
        "focus:ring-purple-300",
        "font-medium",
        "rounded-lg",
        "text-lg",
        "px-5",
        "py-2.5",
        "mb-2",
        "dark:bg-purple-600",
        "dark:hover:bg-purple-700",
        "dark:focus:ring-purple-900",
        "transform",
        "transition",
        "duration-100",
        "hover:translate-x-6",
    ];

    let classes = props.classes.clone();
    html! {
        <Link<Route>
            classes={classes!(button_classes, classes.unwrap_or_default(),)}
            to={props.to.clone()}
        >
            { props.label.clone() }
        </Link<Route>>
    }
}
