use crate::{
    blogs::Post,
    fetch::{fetch_markdown, UrlType},
};
use yew::prelude::*;
use yew_markdown::render_markdown;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PostProps {
    pub post: Post,
}

#[function_component(PostContainer)]
pub fn post(props: &PostProps) -> Html {
    let markdown = use_state(|| "Loading...".to_string());
    let content = props.post.content;
    // async load the content
    {
        let markdown = markdown.clone();
        use_effect_with_deps(
            move |_| {
                let markdown = markdown.clone();
                // if url starts with ./ then fetch file
                // else fetch from url
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_markdown = if content.starts_with("./") {
                        fetch_markdown(content, UrlType::File).await
                    } else {
                        fetch_markdown(content, UrlType::Url).await
                    };
                    match fetched_markdown {
                        Ok(fetched) => markdown.set(fetched),
                        Err(err) => {
                            log::error!("{:?}", err);
                        }
                    };
                });

                // return the teardown
                || ()
            },
            (),
        );
        use_effect(move || {
            crate::highlight();
            || ()
        })
    }
    html! {
        <p class="flex justify-center content-center py-4 bg-[#D682F4]">
            <div class="max-w-4xl p-9">
            {
                render_markdown(
                    (*markdown)
                        .as_str()
                )
            }
            </div>
        </p>
    }
}
