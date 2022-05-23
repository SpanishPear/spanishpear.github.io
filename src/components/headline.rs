use yew::prelude::*;

use crate::{components::link_button::LinkButton, Route};

#[function_component(Headline)]
pub fn headline() -> Html {
    let heading_classes = vec![
        "text-2xl",
        "font-bold",
        "leading-7",
        "dark:text-white",
        "text-gray-900",
        "sm:text-5xl",
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

    html! {
        <div class={classes!(container_classes)}>
            <div class="sm:max-w-l text-center max-w-xl flex items-center justify-center flex-col">
            <h1 class={classes!(heading_classes)}>
                {"Hello! I'm Shrey :)"}
            </h1>
            <p class={classes!(text_classes.clone())} >
                {
                "
                I'm a software engineer based in Sydney, Australia.
                I love learning new things, reading, music,  
                not finishing new side projects, and the colours pink/purple (could you tell?).
                "}
            </p>
            <p class={classes!(text_classes)}>    
                {"
                I'm currently working as a frontend engineer at "} 
                <a class="text-pink-500 background-transparent font-bold uppercase p1-3 py-1 text-xs outline-none focus:outline-none mx-1 mb-1 ease-linear transition-all duration-150" href="https://www.atlassian.com/">
                    {"Atlassian"}
                </a>
                {", and outside
                of that - I help teach, run courses and write software for the
                "}
                <br />
                <a class="text-pink-500 background-transparent font-bold uppercase px-1 py-1 text-xs outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150" href="https://www.unsw.edu.au/engineering/our-schools/computer-science-and-engineering/">{"UNSW School of Computer Science."}</a>
            </p>

            <LinkButton 
                to={ Route::About }    
                label={"Learn more"}
            />
            </div>
        </div>
    }
}
