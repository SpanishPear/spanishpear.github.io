use yew::prelude::*;

use crate::components::{
    headline::Headline,
    recent_posts::RecentPosts,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Headline />
            <RecentPosts />
        </>
    }
}
