use yew::prelude::*;

pub struct Post {
    pub author: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub slug: &'static str,
    pub content: fn() -> Html,
    pub date: &'static str, //sue me
}

pub const POSTS: &[&Post] = &[
    &Post {
        author: "Shrey",
        title: "First post",
        subtitle: "This is the first post",
        slug: "first-post",
        content: || html! {
            <div>
                <h1>{"First Post"}</h1>
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
    }
];
