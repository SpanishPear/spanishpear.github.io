use crate::blogs::Post;
use yew::prelude::*;
use yew_hooks::prelude::*;
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PostProps {
    pub post: Post,
}

#[function_component(PostContainer)]
pub fn post(props: &PostProps) -> Html {
    let content = props.post.content.clone();
    let markdown = use_async(async move { crate::utils::fetch_markdown(content).await });
    html! {}
}
