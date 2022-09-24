use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BackgroundProps {
    pub children: Children,
}

#[function_component(Background)]
pub fn background_wrapper(BackgroundProps { children }: &BackgroundProps) -> Html {
    html! {
        <>
        <div class="flex flex-col justify-around h-full min-h-screen bg-gradient-to-b dark:from-landing-navbar" >
            { children.clone() }
        </div>
        </>
    }
}
