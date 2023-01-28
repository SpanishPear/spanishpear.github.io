/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};
use yew::virtual_dom::{VNode, VTag, VText};
use yew::{html, Classes, Html};

/// Adds a class to the VTag.
/// You can also provide multiple classes separated by ascii whitespaces.
///
/// Note that this has a complexity of O(n),
/// where n is the number of classes already in VTag plus
/// the number of classes to be added.
fn add_class(vtag: &mut VTag, class: impl Into<Classes>) {
    let mut classes: Classes = vtag
        .attributes
        .iter()
        .find(|(k, _)| *k == "class")
        .map(|(_, v)| Classes::from(v.to_owned()))
        .unwrap_or_default();
    classes.push(class);
    vtag.add_attribute("class", classes.to_string());
}

/// Renders a string of Markdown to HTML with the default options (footnotes
/// disabled, tables enabled).
pub fn render_markdown(src: &str) -> Html {
    let mut elems = vec![];
    let mut spine = vec![];

    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            spine[l - 1].add_child($child);
        }};
    }

    let options = Options::all();

    for ev in Parser::new_ext(src, options) {
        match ev {
            Event::Start(tag) => {
                spine.push(make_tag(tag));
            }
            Event::End(tag) => {
                // TODO Verify stack end.
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(_) = tag {
                    let mut pre = VTag::new("pre");
                    pre.add_child(top.into());
                    top = pre;
                } else if let Tag::Table(aligns) = tag {
                    if let Some(top_children) = top.children_mut() {
                        for r in top_children.iter_mut() {
                            if let VNode::VTag(ref mut vtag) = r {
                                if let Some(vtag_children) = vtag.children_mut() {
                                    for (i, c) in vtag_children.iter_mut().enumerate() {
                                        if let VNode::VTag(ref mut vtag) = c {
                                            match aligns[i] {
                                                Alignment::None => {}
                                                Alignment::Left => add_class(vtag, "text-left"),
                                                Alignment::Center => add_class(vtag, "text-center"),
                                                Alignment::Right => add_class(vtag, "text-right"),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Tag::TableHead = tag {
                    if let Some(top_children) = top.children_mut() {
                        for c in top_children.iter_mut() {
                            if let VNode::VTag(ref mut vtag) = c {
                                // TODO
                                //                            vtag.tag = "th".into();
                                vtag.add_attribute("scope", "col");
                            }
                        }
                    }
                }
                if l == 1 {
                    elems.push(top);
                } else {
                    spine[l - 2].add_child(top.into());
                }
            }
            Event::Text(text) => add_child!(VText::new(text.to_string()).into()),
            Event::Rule => add_child!(VTag::new("hr").into()),
            Event::SoftBreak => add_child!(VText::new("\n").into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            Event::Code(text) => {
                let child = VText::new(text.to_string()).into();
                let mut code = VTag::new("code");
                code.add_child(child);
                add_child!(code.into())
            }
            Event::Html(_) => log::error!("HTML not supported"),
            Event::FootnoteReference(inner) => {
                let mut a = VTag::new("a");
                // add link to footnote-[num]
                a.add_attribute("href", format!("#footnote-{inner}"));

                // set the id to footnote-ref-[num], so the footnote itself
                // can link back to this reference
                a.add_attribute("id", format!("footnote-ref-{inner}"));

                // styles
                a.add_attribute("class", "hover:underline");

                a.add_child(VText::new(inner.to_string()).into());
                let mut sup = VTag::new("sup");
                sup.add_child(a.into());
                add_child!(sup.into());
            }
            Event::TaskListMarker(_) => log::error!("TaskListMarker not supported"),
            // HMMM how tf we gonna do html huh
            // Event::Html(text) => add_child!(VText::new(text).into()),
        }
    }

    if elems.len() == 1 {
        VNode::VTag(Box::new(elems.pop().unwrap()))
    } else {
        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

fn make_tag(t: Tag) -> VTag {
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::Heading(n, ..) => {
            let mut el = VTag::new(n.to_string());
            match n {
                HeadingLevel::H1 => {
                    el.add_attribute("class", "text-5xl font-medium my-3 text-center")
                }
                HeadingLevel::H2 => el.add_attribute("class", "text-3xl font-medium my-3"),
                HeadingLevel::H3 => el.add_attribute("class", "text-xl font-medium my-3"),
                HeadingLevel::H4 => el.add_attribute("class", "text-lg font-medium my-3"),
                HeadingLevel::H5 => el.add_attribute("class", "text-lg font-medium my-3"),
                HeadingLevel::H6 => el.add_attribute("class", "text-sm font-medium my-3"),
            }
            el
        }
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_attribute("class", "p-4 my-4 bg-gray-50 italic border-l-4 border-gray-300 dark:border-gray-500 dark:bg-gray-800 dark:text-white");
            el
        }
        Tag::CodeBlock(code_block_kind) => {
            let mut el = VTag::new("code");

            if let CodeBlockKind::Fenced(lang) = code_block_kind {
                // Different color schemes may be used for different code blocks,
                // but a different library (likely js based at the moment) would be necessary to
                // actually provide the highlighting support by locating the
                // language classes and applying dom transforms on their contents.
                match lang.as_ref() {
                    "html" => el.add_attribute("class", "language-html"),
                    "rust" => el.add_attribute("class", "language-rust"),
                    "java" => el.add_attribute("class", "language-java"),
                    "c" => el.add_attribute("class", "language-c"),
                    "bash" => el.add_attribute("class", "language-bash"),
                    _ => {} // Add your own language highlighting support
                };
            }
            el.add_attribute("class", "my-3");
            el
        }
        Tag::List(None) => {
            let mut el = VTag::new("ul");
            el.add_attribute("class", "list-disc list-inside");
            el
        }
        Tag::List(Some(1)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("class", "list-decimal list-inside");
            el
        }
        Tag::List(Some(ref start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start.to_string());
            el.add_attribute("class", "list-decimal list-inside");
            el
        }
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            el.add_attribute("class", "table");
            el
        }
        Tag::TableHead => VTag::new("th"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "font-italic");
            el
        }
        Tag::Strong => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "font-bold");
            el
        }
        Tag::Link(_link_type, ref href, ref title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href.to_string());
            let title = title.clone().into_string();
            if !title.is_empty() {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::Image(_link_type, ref src, ref title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src.to_string());
            let title = title.clone().into_string();
            if !title.is_empty() {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::FootnoteDefinition(ref footnote_id) => {
            // display the footnote id as a link to the original reference
            // and then the footnote contents
            let mut el = VTag::new("div");
            el.add_attribute("id", format!("footnote-{footnote_id}"));
            el.add_attribute("class", "my-3");

            let mut a = VTag::new("a");
            a.add_attribute("href", format!("#footnote-ref-{footnote_id}"));
            a.add_attribute("class", "hover:underline");
            a.add_child(VText::new(format!("⬑ {footnote_id}: ")).into());
            let mut sup = VTag::new("strong");
            sup.add_child(a.into());
            el.add_child(sup.into());

            el
        }
        // rendered as anything
        // special
        Tag::Strikethrough => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "line-through");
            el
        }
    }
}
