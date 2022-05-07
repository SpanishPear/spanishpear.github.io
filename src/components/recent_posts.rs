use yew::prelude::*;

use crate::blogs::POSTS;


#[function_component(RecentPosts)]
pub fn recent_posts() -> Html {

    html! {
        
        POSTS.iter().enumerate().map(|(i, post)| {
            if i < 3 {
                html! {
                    <div class="post-preview">
                        <a>
                            <h2 class="post-title">
                                { post.title.clone() }
                            </h2>
                            <h3 class="post-subtitle">
                                { post.subtitle.clone() }
                            </h3>
                        </a>
                        <p class="post-meta">{"Posted by"}
                            <a href="#">{ post.author.clone() }</a>
                            { format!(" on {}", post.date) }
                        </p>
                    </div>
                }
            } else {
                html! {}
            }
        }).collect::<Html>()

    }



}
