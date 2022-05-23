use yew::prelude::*;
use crate::components::navbar::Navbar;

#[derive(Clone, Properties, PartialEq)]
pub struct BackgroundProps {
   pub children: Children,
}

#[function_component(Background)]
pub fn background_wrapper(props: &BackgroundProps) -> Html {
    
    html! {
        <>
        <Navbar />
        <div class="min-h-[93vh] h-full bg-gradient-to-b dark:from-landing-navbar dark:to-landing-container-end" >
            { props.children.clone() }
        </div>
        </>
    }

}
