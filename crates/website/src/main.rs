#![allow(clippy::let_unit_value)]
use components::gradient_background::Background;
use components::navbar::Navbar;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;
use yew_router::prelude::*;

use self::blogs::{Post, Tags};

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
    yew::Renderer::<App>::new().render();
}

fn switch(route: Route, posts: UseStateHandle<Vec<Post>>) -> Html {
    match route {
        Route::Home => html! {
            <Background>
                <pages::home::Home />
            </Background>
        },
        Route::NotFound => html! { <pages::not_found::NotFound /> },
        Route::Contact => html! { <pages::construction::Construction /> },
        Route::Blog => html! { <pages::all_posts::PostPage /> },
        Route::BlogPost { id } => {
            let post = posts.iter().find(|post| post.slug == *id);
            let post = match post {
                Some(post) => post.clone(),
                None => blogs::Post {
                    author: "".into(),
                    title: "404".into(),
                    subtitle: "".into(),
                    slug: "".into(),
                    content: "404".into(),
                    date: "".into(),
                    public: true,
                    thumbnail_path: "".into(),
                    tags: None,
                },
            };

            html! {
                <pages::post::PostContainer post={post} />
            }
        }
        Route::Projects => html! { <pages::construction::Construction /> },
        Route::Project { id: _ } => html! { <pages::construction::Construction /> },
    }
}
#[function_component(ScrollToTop)]
fn scroll_to_top() -> Html {
    // this is just a component that will scroll_to_top
    // when the route changes
    let location = use_location();
    let pathname = match location {
        Some(location) => {
            location.path().to_string()
        },
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
    let posts = use_state(|| {
        vec![
            Post {
                author: "Shrey".into(),
                title: "A prettier terminal".into(),
                subtitle: "maybe you'll actually enjoy using it".into(),
                slug: "prettier-terminal".into(),
                content: "./posts/making_a_prettier_terminal.md".into(),
                date: "2022-05-07".into(),
                thumbnail_path: "/assets/pretty-terminal.png".into(),
                public: true,
                tags: Some(Tags::new(vec!["tooling"])),
            },
            Post {
                author: "Shrey".into(),
                title: "Smart Pointers".into(),
                subtitle: "A deep dive into the Rust smart pointer types".into(),
                slug: "smart-pointers".into(),
                content: "./posts/smart_pointers.md".into(),
                date: "2022-24-09".into(),
                thumbnail_path: "/assets/smart-pointers.png".into(),
                public: false,
                tags: None,
            },
            Post {
                author: "Shrey".into(),
                title: "Hustle Mania".into(),
                subtitle: "Burnout in the world of tech".into(),
                slug: "hustle_mania".into(),
                content: "./posts/hustle_mania.md".into(),
                date: "2022-24-12".into(),
                thumbnail_path: "".into(),
                // lmao you thought
                public: false,
                tags: None,
            },
            Post {
                author: "Shrey".into(),
                title: "\"Graduating\"".into(),
                subtitle: "I've finished my degree, so why doesn't it feel special?".into(),
                slug: "graduating".into(),
                content: "./posts/graduating.md".into(),
                date: "2022-24-12".into(),
                thumbnail_path:
                    "https://cdn.pixabay.com/photo/2016/03/19/04/40/cap-1266204__340.png".into(),
                public: true,
                tags: Some(Tags::new(vec!["life"])),
            },
        ]
    });

    html! {
        <ContextProvider<Vec<Post>> context={(*posts).clone()}>
            <ContextProvider<Theme> context={(*theme).clone()}>
                <BrowserRouter>
                    <Navbar />
                    <ScrollToTop />
                    <Switch<Route> render={move |route| switch(route, posts.clone())} />
                </BrowserRouter>
            </ContextProvider<Theme>>
        </ContextProvider<Vec<Post>>>
    }
}
