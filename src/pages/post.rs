use crate::blogs::Post;
use yew::prelude::*;


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PostProps {
    pub post: Post,
}

#[function_component(PostContainer)]
pub fn post(props: &PostProps) -> Html {
    let markdown = crate::utils::fetch_markdown(props.post.content).await?;
    html!{}
}
