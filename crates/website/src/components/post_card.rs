use crate::blogs::{Post, Tags};
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct PostCardProps {
    pub post: Post,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    html! {
        <Link<Route> to={Route::BlogPost { id: props.post.slug.to_string() }}>
            <div class="w-full transform transition duration-100 hover:scale-110 hover:cursor-pointer m-1 max-w-xl bg-white rounded-lg border-gray-200 shadow-md dark:bg-gray-800 dark:border-gray-700">
                <img class="rounded-t-lg" src={props.post.thumbnail_path.clone()} alt="" />
                <div class="p-5">
                    <Link<Route> to={Route::BlogPost { id: props.post.slug.to_string() }}>
                        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                            {props.post.title.clone()}
                        </h5>
                    </Link<Route>>
                    <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">
                        {props.post.subtitle.clone()}
                        // add a dot and then the tags
                        {format!(" • {}", props.post.tags.clone().unwrap_or_else(|| Tags::new(vec![])).join(", "))}

                    </p>
                    <Link<Route> to={Route::BlogPost { id: props.post.slug.to_string() }} classes="inline-flex items-center py-2 px-3 text-sm font-medium text-center text-white bg-purple-700 rounded-lg hover:bg-purple-800 focus:ring-4 focus:outline-none focus:ring-purple-300 dark:bg-purple-600 dark:hover:bg-purple-700 dark:focus:ring-purple-800">
                        {"Read more"}
                        <svg class="ml-2 -mr-1 w-4 h-4" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                    </Link<Route>>
                </div>
            </div>
        </Link<Route>>
    }
}
