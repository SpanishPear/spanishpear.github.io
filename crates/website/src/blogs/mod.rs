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
    pub tags: Option<Tags>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tags(Vec<TagMembers>);

impl Tags {
    pub fn new(tags: Vec<&str>) -> Self {
        Self(
            tags.iter()
                .map(|&tag| tag.into())
                .collect::<Vec<TagMembers>>(),
        )
    }

    pub fn join(&self, separator: &str) -> String {
        let members = self.0.iter().map(|m| m.to_string()).collect::<Vec<_>>();
        members.join(separator)
    }
}

impl Default for Tags {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Iterator for Tags {
    type Item = TagMembers;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl ToString for Tags {
    fn to_string(&self) -> String {
        self.join(", ")
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TagMembers {
    Rust,
    Tooling,
    WASM,
}

impl From<&str> for TagMembers {
    fn from(tag: &str) -> Self {
        match tag {
            "rust" => Self::Rust,
            "tooling" => Self::Tooling,
            "wasm" => Self::WASM,
            _ => panic!("Unknown tag: {}", tag),
        }
    }
}

impl ToString for TagMembers {
    fn to_string(&self) -> String {
        match self {
            TagMembers::Rust => "Rust".to_string(),
            TagMembers::Tooling => "Tooling".to_string(),
            TagMembers::WASM => "WASM".to_string(),
        }
    }
}
