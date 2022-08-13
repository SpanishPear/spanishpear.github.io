use yew::prelude::*;
use crate::{Route, blogs::POSTS, components::link_button::LinkButton, components::post_card::PostCard};

#[function_component(RecentPosts)]
pub fn recent_posts() -> Html {
    html! {
        <div class="p-12 dark:bg-landing-container-end flex flex-col items-center">
            <div class="flex w-10/12 flex-col items-center">
                <div class="md:max-w-[75%] xs:max-w-full md:p-6 flex w-full items-center justify-between flex-row">
                    <h2 class="text-sm md:text-xl rounded sm:p-2 text-white md:text-left text-center">
                        {"Recent Posts"}
                    </h2>
                    <LinkButton to={Route::Blog} label={"View All →"} />
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
