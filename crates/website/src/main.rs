use components::gradient_background::Background;
use components::navbar::Navbar;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;
use yew_router::prelude::*;

#[macro_use]
pub mod macros;
pub mod blogs;
pub mod components;
pub mod fetch;
pub mod pages;

#[wasm_bindgen]
extern "C" {
    pub fn highlight();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    dark: bool,
}

#[derive(Debug, Clone, Routable, PartialEq, Eq)]
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
        Route::Home => html! {
            <Background>
                <pages::home::Home />
            </Background>
        },
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

            html! {
                <pages::post::PostContainer {post} />
            }
        }
        Route::Projects => html! { <pages::construction::Construction /> },
        Route::Project { id: _ } => html! { <pages::construction::Construction /> },
    }
}
#[function_component(ScrollToTop)]
fn scroll_to_top() -> Html {
    let location = use_location();
    let pathname = match location {
        Some(AnyLocation::Browser(location)) => location.pathname(),
        None => "".to_string(),
    };
    {
        let pathname2 = pathname.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    if pathname != "/" {
                        web_sys::window().unwrap().scroll_to_with_scroll_to_options(
                            web_sys::ScrollToOptions::new().top(0.0),
                        );
                    }
                });
                || ()
            },
            pathname2,
        );
    }
    html! {}
}
#[function_component(App)]
fn app() -> Html {
    let theme = use_state(|| Theme { dark: false });
    html! {
        <ContextProvider<Theme> context={(*theme).clone()}>
            <BrowserRouter>
                <Navbar />
                <ScrollToTop />
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<Theme>>
    }
}
