use yew::Properties;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Post {
    pub author: String,
    pub title: String,
    pub subtitle: String,
    pub slug: String,
    pub content: String,
    pub date: String, //sue me
    pub thumbnail_path: String,
    pub tags: Option<Vec<Tags>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Tags {
    Rust,
}
