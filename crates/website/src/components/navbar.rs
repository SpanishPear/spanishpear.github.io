use crate::Route;
use yew::prelude::*;
use yew_router::{hooks::use_route, prelude::*};

#[derive(Properties, Debug, PartialEq)]
struct NavItem {
    link: Route,
    text: &'static str,
}

const NAV_CLASSES: &[&str] = &[
    "block",
    "py-2",
    "pr-4",
    "pl-3",
    "text-black",
    "rounded",
    "md:bg-transparent",
    "md:p-0",
    "dark:text-white",
    "hover:underline",
    "text-2xl",
];

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let nav_items = vec![
        NavItem {
            link: Route::Home,
            text: "Home",
        },
        NavItem {
            link: Route::About,
            text: "About",
        },
        NavItem {
            link: Route::Blog,
            text: "Blog",
        },
        NavItem {
            link: Route::Projects,
            text: "Projects",
        },
        NavItem {
            link: Route::Contact,
            text: "Contact",
        },
    ];

    let mobile_navbar_showing = use_state(|| false);
    let mobile_onclick = {
        let mobile_navbar_showing = mobile_navbar_showing.clone();
        Callback::from(move |_| {
            mobile_navbar_showing.set(!*mobile_navbar_showing);
        })
    };

    html! {
        <nav class="dark:bg-landing-navbar bg-white border-gray-200 px-2 sm:px-4 py-8">
          <div class="container flex flex-wrap justify-between items-center mx-auto">
            <Link<Route> to={Route::Home} classes="flex items-center">
                <img loading="lazy" src="/assets/logo.svg" class="mr-3 h-9 sm:h-12" width="77" height="67" alt="SpanishPear Icon" />
            </Link<Route>>
            <button data-collapse-toggle="mobile-menu" onclick={mobile_onclick} class="sm:hidden inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="mobile-menu" aria-expanded="false">
              <span class="sr-only">{"Open Main Menu"}</span>
              <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg>
              <svg class="hidden w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
            </button>
            <div class={classes!("w-full", "md:block", "md:w-auto", if *mobile_navbar_showing {"block"} else {"hidden"})} id="mobile-menu">
              <ul class="flex flex-col items-center mt-4 md:flex-row md:space-x-8 md:mt-0 md:text-base md:font-medium">
                {
                    nav_items.iter().map(|item| {
                        html! {
                            <NavItemComponent key={item.text} link={item.link.clone()} text={item.text} />
                        }
                    }).collect::<Html>()
                }
                <a href="javascript:;" title="whats this?" class="hover:cursor-pointer hover:border-purple-300 border-transparent border-2 hover:border-current rounded">
                    <img loading="lazy" src="/assets/sparkles_icon.svg" class="hover:cursor-pointer h-2 hover:invert sm:h-12 " width="32" height="20" alt="sparkles" />
                </a>
              </ul>
            </div>
          </div>
        </nav>
    }
}

#[function_component(NavItemComponent)]
fn navbar_item(NavItem { link, text }: &NavItem) -> Html {
    let route: Option<Route> = use_route();

    html! {
        <li>
          <Link<Route> to={link.clone()} classes={
              classes!(NAV_CLASSES, route.map(|r| {
                  if let Route::BlogPost { id: _ } = r {
                      if link == &Route::Blog {
                        "underline"
                      } else {
                        ""
                      }
                 } else if r == *link { "underline" } else { "" }
              }))
            }
          >
              {text}
          </Link<Route>>
        </li>
    }
}
