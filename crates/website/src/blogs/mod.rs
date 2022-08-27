#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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
        content: "https://raw.githubusercontent.com/SpanishPear/spanishpear.github.io/main/posts/making_a_prettier_terminal.md",
        date: "2022-05-07",
        thumbnail_path: "/assets/pretty-terminal.png",
    }
];
