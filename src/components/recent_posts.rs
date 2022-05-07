use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::{blogs::POSTS, components::link_button::LinkButton, components::post_card::PostCard};

#[function_component(RecentPosts)]
pub fn recent_posts() -> Html {
    let history = use_history().expect("No history");
    let goto_blog = Callback::once(move |_| history.push(Route::Blog));

    html! {
        <div class="p-12 dark:bg-landing-container-end flex flex-col items-center">
            <div class="flex w-10/12 flex-col">
            <div class="p-6 flex w-full justify-between items-center flex-row">
                <h2 class="text-xl rounded p-2 text-white">{"Recent Posts"}</h2>
                <LinkButton onclick={goto_blog} label={"View All →"} />
            </div>
            <div class="flex flex-row w-full justify-around">
            {
                POSTS.iter().enumerate().map(|(i, &post)| {
                    html! {
                        if i < 1 {
                            <PostCard post={*post} />
                        } else {
                            <></>
                        }
                    }
                }).collect::<Html>()
            }
            </div>
            </div>
        </div>
    }
}
