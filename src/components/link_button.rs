use yew::prelude::*;


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ButtonProps {
    pub label: String,
    pub onclick: yew::Callback<yew::MouseEvent>,
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
        "text-sm",
        "px-5",
        "py-2.5",
        "mb-2",
        "dark:bg-purple-600",
        "dark:hover:bg-purple-700",
        "dark:focus:ring-purple-900",
        "transform",
        "transition",
        "duration-100",
        "hover:translate-x-6" 
    ];
    
    html! {
        <a href="javascript:;" 
            class={classes!(button_classes)} 
            onclick={props.onclick.clone()}
        > 
            {props.label.clone()}
        </a>
    }
}
