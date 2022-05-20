use yew::prelude::*;
pub mod prettier_terminal;


#[derive(Clone, Copy,  PartialEq)]
pub struct Post {
    pub author: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub slug: &'static str,
    pub content: fn() -> Html,
    pub date: &'static str, //sue me
    pub thumbnail_path: &'static str,
}

pub const POSTS: &[&Post] = &[
    &Post {
        author: "Shrey",
        title: "A prettier terminal",
        subtitle: "maybe you'll actually enjoy using it",
        slug: "prettier-terminal",
        content: prettier_terminal::content,
        date: "2022-05-07",
        thumbnail_path: "/assets/pretty-terminal.png",
    }, 
    &Post {
        author: "Shrey",
        title: "My First post",
        subtitle: "This is the first post",
        slug: "first-post",
        content: || html! {
            <div>
                <h1>{"Post"}</h1>
                <p>
                    {"
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                    Mauris euismod, nisl eu aliquam consectetur, nisl nisl
                    aliquet nunc, eu porttitor nisl nisl eu nisl.
                    "}
                </p>
            </div>
        },
        date: "2022-05-07",
        thumbnail_path: "/assets/pretty-terminal.png",
    }, 
    &Post {
        author: "Shrey",
        title: "My First post",
        subtitle: "This is the first post",
        slug: "first-post",
        content: || html! {
            <div>
                <h1>{"Post"}</h1>
                <p>
                    {"
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                    Mauris euismod, nisl eu aliquam consectetur, nisl nisl
                    aliquet nunc, eu porttitor nisl nisl eu nisl.
                    "}
                </p>
            </div>
        },
        date: "2022-05-07",
        thumbnail_path: "/assets/pretty-terminal.png",
    }

];
