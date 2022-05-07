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
    #[at("/blog/{id}")]
    BlogPost { id: String },
    #[at("/projects")]
    Projects,
    #[at("/projects/{id}")]
    Project { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <pages::home::Home /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::About => html! { <h1>{ "About" }</h1> },
        Route::Contact => html! { <h1>{ "Contact" }</h1> },
        Route::Blog => html! { <h1>{ "Blog" }</h1> },
        Route::BlogPost { id } => html! { <h1>{ format!("Blog Post {}", id) }</h1> },
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
