use yew::prelude::*;
pub mod prettier_terminal;

const MARKDOWN_URL: &str = "https://raw.githubusercontent.com/SpanishPear/portfolio/main/src/assets/blogs/making_a_prettier_terminal.md";

#[derive(Clone, Debug, Copy,  PartialEq)]
pub struct Post {
    pub author: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub slug: &'static str,
    pub content: &'static str,
    pub date: &'static str, //sue me
    pub thumbnail_path: &'static str,
}

pub const POSTS: &[&Post] = &[
    &Post {
        author: "Shrey",
        title: "A prettier terminal",
        subtitle: "maybe you'll actually enjoy using it",
        slug: "prettier-terminal",
        content: MARKDOWN_URL,
        date: "2022-05-07",
        thumbnail_path: "/assets/pretty-terminal.png",
    }
];
