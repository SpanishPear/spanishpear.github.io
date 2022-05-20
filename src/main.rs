use yew_router::prelude::*;
use yew::prelude::*;


#[macro_use]
pub mod utils;
pub mod components;
pub mod pages;
pub mod blogs;


use components::{background::Background, navbar::Navbar};

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
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::About => html! { <h1>{ "About" }</h1> },
        Route::Contact => html! { <h1>{ "Contact" }</h1> },
        Route::Blog => html! { <h1>{ "Blog" }</h1> },
        Route::BlogPost { id } => {
            
            let post = crate::blogs::POSTS.iter().find(|post| post.slug == id).unwrap_or({
                &&crate::blogs::Post {
                    author: "",
                    title: "404",
                    subtitle: "",
                    slug: "404",
                    content: || {
                        html! {
                            <div>
                                <h1>{ "404" }</h1>
                                <p>{ "Post not found" }</p>
                            </div>
                        }
                    },
                    date: "",
                    thumbnail_path: "",
                }
            });

            (post.content)()
            
        },
        Route::Projects => html! { <h1>{ "Projects" }</h1> },
        Route::Project { id } => html! { <h1>{ format!("Project {}", id) }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    
    let theme = use_state(|| Theme { dark: false });
    
    
    html! {
        <ContextProvider<Theme> context={(*theme).clone()}>
            <BrowserRouter>
                <Background>
                    <Navbar />
                    <Switch<Route> render={Switch::render(switch)} />
                </Background>
            </BrowserRouter>
        </ContextProvider<Theme>>
    }
}
