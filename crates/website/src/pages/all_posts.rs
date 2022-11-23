use yew::prelude::*;

use crate::blogs::Post;
use crate::components::post_card::PostCard;

#[function_component(PostPage)]
pub fn render() -> Html {
    let posts = use_context::<Vec<Post>>().unwrap();

    html! {
        <div class="flex justify-center content-center py-4 bg-[#D682F4]">
            // show all posts
            <div class="flex flex-col justify-center content-center w-full max-w-4xl">
                <h1 class="text-4xl font-bold text-center text-white">{"All Posts"}</h1>
                <div class="mt-3 flex flex-col justify-center content-center items-center w-full">
                    // show post card but reverse time
                    {
                        for posts.iter().rev().map(|post| {
                            if post.public {
                                html! {
                                    <PostCard post={post.clone()} />
                                }
                            } else {
                                html! {}
                            }
                        })
                    }
                </div>
            </div>
        </div>
    }
}
