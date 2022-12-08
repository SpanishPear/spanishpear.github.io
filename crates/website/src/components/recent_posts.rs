use crate::{
    blogs::Post, components::link_button::LinkButton, components::post_card::PostCard, Route,
};
use yew::prelude::*;

#[function_component(RecentPosts)]
pub fn recent_posts() -> Html {
    let inner_container_styles = vec![
        "flex",
        "sm:w-full",
        "lg:w-7/12",
        "flex-col",
        "items-center",
        "pb-6",
    ];
    let posts = use_context::<Vec<Post>>().expect("no posts found");
    html! {
        <div class="p-12 mt-16 flex flex-col items-center">
            <div class={classes!(inner_container_styles)}>
                <RecentPostHeader />
                <div class="flex flex-row w-full justify-center lg:max-w-[75%] sm:max-w-full">
                {
                    posts.iter().filter(|post| post.public).last().map(|post| {
                        html!{
                            <PostCard post={post.clone()} />
                        }
                    }).unwrap_or_else(|| html!{})
                }
                </div>
            </div>
        </div>
    }
}

#[function_component(RecentPostHeader)]
pub fn recent_post_header() -> Html {
    let container_styles = vec![
        "lg:max-w-[75%]",
        "sm:max-w-full",
        "md:p-3",
        "flex",
        "w-full",
        "items-center",
        "justify-between",
        "flex-row",
    ];

    let heading_styles = vec![
        "text-sm",
        "underline-offset-8",
        "underline",
        "decoration-landing-container-end",
        "md:text-2xl",
        "rounded",
        "sm:p-2",
        "text-white",
        "md:text-left",
        "text-center",
    ];

    html! {
        <div class={classes!(container_styles)}>
            <h2 class={classes!(heading_styles)}>
                {"Recent Posts"}
            </h2>
            <LinkButton to={Route::Blog} label={"View All →"} />
        </div>
    }
}
