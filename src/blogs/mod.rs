use yew::prelude::*;


struct Post {
    author: String,
    title: String,
    subtitle: String,
    slug: String,
    content: Html,
}

const POSTS: &[&Post] = [
    Post {
        author: "Shrey".to_string(),
        title: "First post",
        subtitle: "This is the first post",
        slug: "first-post",
        content: html! {
            <div>
                <h1>First Post</h1>
                <p>
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                    Mauris euismod, nisl eu aliquam consectetur, nisl nisl
                    aliquet nunc, eu porttitor nisl nisl eu nisl.
                </p>
            </div>
        },
    }
];
