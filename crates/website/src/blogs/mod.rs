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
        content: "./posts/making_a_prettier_terminal.md",
        date: "2022-05-07",
        thumbnail_path: "/assets/pretty-terminal.png",
    },
    &Post {
        author: "Shrey",
        title: "Smart Pointers",
        subtitle: "A deep dive into the Rust smart pointer types",
        slug: "smart-pointers",
        content: "./posts/smart_pointers.md",
        date: "2022-24-09",
        thumbnail_path: "/assets/smart-pointers.png",
    },
];
