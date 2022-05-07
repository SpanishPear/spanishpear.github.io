use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Headline)]
pub fn headline() -> Html {
    let history = use_history().expect("Browser history should exist");
    let heading_classes = vec![
        "text-2xl",
        "font-bold",
        "leading-7",
        "dark:text-white",
        "text-gray-900",
        "sm:text-3xl",
        "sm:truncate",
    ];

    let container_classes = vec![
        "flex",
        "items-center",
        "justify-center",
        "flex-col",
        "h-full",
        "w-full",
        "py-48",
        "bg-gradient-to-b",
        "dark:from-landing-navbar",
        "dark:to-landing-container-end",
    ];

    let text_classes = vec![
        "text-white",
        "my-4",
    ];





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
    ];
    html! {
        <div class={classes!(container_classes)}>
            <div class="sm:max-w-l text-center max-w-xl flex items-center justify-center flex-col">
            <h1 class={classes!(heading_classes)}>
                {"Welcome! I'm Shrey :)"}
            </h1>
            <p class={classes!(text_classes.clone())} >
                {
                "
                I'm a software engineer based in Sydney, Australia.
                I love learning new things, reading, music and 
                not finishing new side projects.
                "}
            </p>
            <p class={classes!(text_classes)}>    
                {"
                I'm currently working as a full-stack engineer at Atlassian, and outside
                of that - I help teach, run courses and write software for 
                "}
                <a href="https://www.unsw.edu.au/engineering/our-schools/computer-science-and-engineering/">{"UNSW School of Computer Science."}</a>
            </p>
            <a href="javascript:;" class={classes!(button_classes)} onclick={
                Callback::once(move |_| history.push(Route::About))
            }>
                {"Learn more"}
            </a>
            </div>
        </div>
    }
}
