use yew::prelude::*;

use crate::{components::link_button::LinkButton, Route};

#[function_component(Headline)]
pub fn headline() -> Html {
    let heading_classes = vec![
        "text-5xl",
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
        "h-[31em]",
        "w-full",
        "py-38",
        "xs:px-2",
        "md:px-0",
        "bg-gradient-to-b",
        "dark:from-landing-navbar",
    ];

    let text_classes = vec!["text-white", "my-4", "xs:text-lg", "text-3xl"];

    let link_classes = vec![
        "text-[#fde047]",
        "background-transparent",
        "font-bold",
        "uppercase",
        "px-1",
        "py-1",
        "outline-none",
        "focus:outline-none",
        "mr-1",
        "mb-1",
        "text-xl",
    ];

    html! {
        <div class={classes!(container_classes)}>
            <div class="sm:max-w-l xs:px-2 text-center max-w-3xl flex items-center justify-center flex-col">
            <h1 class={classes!(heading_classes, "my-10")}>
                {"Hello! I'm Shrey :)"}
            </h1>
            <br class="mt-14"/>
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
                <a class={classes!(link_classes.clone())}
                    href="https://www.atlassian.com/">
                    {"Atlassian"}
                </a>
                {", and outside
                of that - I help teach, run courses and write software for the
                "}
                <br />
                <a class={classes!(link_classes)}
                    href="https://www.unsw.edu.au/engineering/our-schools/computer-science-and-engineering/">
                    {"UNSW School of Computer Science."}
                </a>
            </p>

            <LinkButton
                to={ Route::About }
                label={"Learn more"}
            />
            </div>
        </div>
    }
}
