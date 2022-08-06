use crate::{fetch, blogs::Post};
use yew::prelude::*;
use yew_hooks::prelude::*;
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PostProps {
    pub post: Post,
}

#[function_component(PostContainer)]
pub fn post(props: &PostProps) -> Html {
    let markdown = use_state(|| "Loading...".to_string());
    let content = props.post.content.clone();
    // async load the content
    { 
        let markdown = markdown.clone();
        use_effect_with_deps(move |_| {
            let markdown = markdown.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_markdown = fetch::fetch_markdown_from_url(content).await;
                match fetched_markdown {
                    Ok(fetched) => markdown.set(fetched),
                    Err(err) => {
                        log::error!("{:?}", err);
                    }
                };
            });

            // return the teardown
            || ()
        }, ());
    }
    html! {
        <p> {yew_markdown::render_markdown(&*markdown)} </p>
    }
}
