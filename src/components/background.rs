use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BackgroundProps {
   pub children: Children,
}

#[function_component(Background)]
pub fn background_wrapper(props: &BackgroundProps) -> Html {

    html! {
        <div>
            { props.children.clone() }
        </div>
    }

}
