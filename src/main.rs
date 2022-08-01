use yew::prelude::*;
use yew_router::prelude::*;

#[macro_use]
pub mod macros;
pub mod utils;
pub mod blogs;
pub mod components;
pub mod pages;

use components::background::Background;

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    dark: bool,
}

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogPost { id: String },
    #[at("/projects")]
    Projects,
    #[at("/projects/:id")]
    Project { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <pages::home::Home /> },
        Route::NotFound => html! { <pages::not_found::NotFound /> },
        Route::About => html! { <pages::construction::Construction /> },
        Route::Contact => html! { <pages::construction::Construction /> },
        Route::Blog => html! { <pages::construction::Construction /> },
        Route::BlogPost { id } => {
            let post = **crate::blogs::POSTS
                .iter()
                .find(|post| post.slug == id)
                .unwrap_or({
                    &&crate::blogs::Post {
                        author: "",
                        title: "404",
                        subtitle: "",
                        slug: "404",
                        content: "",
                        date: "",
                        thumbnail_path: "",
                    }
                });

            html! { <pages::post::PostContainer {post} /> }
        }
        Route::Projects => html! { <pages::construction::Construction /> },
        Route::Project { id: _} => html! { <pages::construction::Construction /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let theme = use_state(|| Theme { dark: false });
    html! {
        <ContextProvider<Theme> context={(*theme).clone()}>
            <BrowserRouter>
                <Background>
                    <Switch<Route> render={Switch::render(switch)} />
                </Background>
            </BrowserRouter>
        </ContextProvider<Theme>>
    }
}
