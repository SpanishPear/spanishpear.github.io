use crate::Route;
use crate::{blogs::Post, fetch::fetch_url};
use yew::prelude::*;
use yew_markdown::render_markdown;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PostProps {
    pub post: Post,
}

#[function_component(PostContainer)]
pub fn post(props: &PostProps) -> Html {
    let markdown = use_state(|| "Loading...".to_string());
    let content = props.post.content.clone();

    let navigator = use_navigator().expect("should have navigator"); 

    if content == "404" {
        log::trace!("404: Given post slug does not exist");
        navigator.push(&Route::NotFound)
    }

    // async load the content
    {
        let markdown = markdown.clone();
        use_effect_with_deps(
            move |_| {
                let markdown = markdown.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_url(&content).await {
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
            <div class="md:max-w-4xl max-w-[100vw] p-9">
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
