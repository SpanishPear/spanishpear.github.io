use yew::prelude::*;
use crate::components::navbar::Navbar;

#[derive(Clone, Properties, PartialEq)]
pub struct BackgroundProps {
   pub children: Children,
}

#[function_component(Background)]
pub fn background_wrapper(BackgroundProps { children }: &BackgroundProps) -> Html {
    html! {
        <>
        <Navbar />
        <div class="h-full min-h-screen bg-gradient-to-b dark:from-landing-navbar dark:to-landing-container-end" >
            { children.clone() }
        </div>
        </>
    }

}
