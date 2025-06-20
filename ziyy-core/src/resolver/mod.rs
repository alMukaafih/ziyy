use std::{collections::HashMap, rc::Rc};

use crate::{
    BUILTIN_TAGS,
    common::Span,
    parser::{
        chunk::{Chunk, ChunkData},
        tag_parer::tag::{Tag, TagType},
    },
};
use document::{Document, Node};

pub mod document;

#[doc(hidden)]
pub struct Resolver {}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Resolver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn resolve(&mut self, chunks: Vec<Chunk>) -> Rc<Document> {
        let tree = Document::new();
        let mut node = tree.root();

        for chunk in chunks.iter() {
            match &chunk.data {
                ChunkData::Tag(tag) => match tag.r#type {
                    TagType::Open => {
                        node = node.append(chunk.clone());
                    }

                    TagType::Close => {
                        node.append(chunk.clone());
                        node = node.parent().unwrap();
                    }

                    TagType::SelfClose => {
                        node.append(chunk.clone());
                    }
                },
                ChunkData::WhiteSpace(_) => {
                    node.append(chunk.clone());
                }
                ChunkData::Word(_) => {
                    node.append(chunk.clone());
                }
                ChunkData::Ansi(_) => {
                    node.append(chunk.clone());
                }
            }
        }

        let node = tree.root();

        {
            let mut detachables = vec![];
            let mut bindings: HashMap<String, Tag> = HashMap::new();
            Resolver::resolve_bindings(&mut bindings, &node, &mut detachables);
            for node in &detachables {
                node.detach();
            }
        }

        {
            let mut detachables = vec![];
            Resolver::optimize_ws(&node, &mut detachables);
            for node in &detachables {
                node.detach();
            }
        }

        Resolver::_resolve(&node, "$root");

        tree
    }

    /// Resolve all declared bindings: <let />
    fn resolve_bindings(
        bindings: &mut HashMap<String, Tag>,
        node: &Rc<Node>,
        detachables: &mut Vec<Rc<Node>>,
    ) {
        for child in node.children() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_tag() {
                let tag = child_chunk.tag_mut().unwrap();
                let name = tag.name().clone();

                if !BUILTIN_TAGS.contains(&name.as_str()) {
                    for ansector in child.ancestors() {
                        if let Some(binding) = bindings.get(&format!("{}/{}", ansector.id(), name))
                        {
                            tag.inherit(binding);
                            break;
                        }
                    }
                }

                if !tag.src().is_empty() {
                    for ansector in child.ancestors() {
                        if let Some(binding) =
                            bindings.get(&format!("{}/{}", ansector.id(), tag.src()))
                        {
                            tag.inherit(binding);
                            break;
                        }
                    }
                }

                if name == "let" {
                    let tag = tag.clone();
                    let name = tag.custom();
                    let id = node.id();
                    bindings.insert(format!("{id}/{name}"), tag);
                    detachables.push(child.clone());
                }
            }
            Resolver::resolve_bindings(bindings, &child, detachables);
        }
    }

    /// Optimizes Excess Whitespace
    fn optimize_ws(node: &Rc<Node>, detachables: &mut Vec<Rc<Node>>) {
        for child in node.children() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_ws() {
                if child.id() == 1 {
                    detachables.push(child.clone());
                } else if child.id() as usize == child.doc().len() - 1
                    && child_chunk.ws().is_some_and(|s| s.contains("\n"))
                {
                    child_chunk.data = ChunkData::WhiteSpace("\n".to_string());
                } else {
                    child_chunk.data = ChunkData::WhiteSpace(" ".to_string());
                }

                if let Some(first) = child.next_sibling().and_then(|next| next.first_child()) {
                    if first.chunk().borrow().is_ws() {
                        detachables.push(first);
                    }
                } else if child.next_sibling().is_some_and(|node| {
                    node.chunk()
                        .borrow()
                        .is_tag_and(|tag| tag.r#type == TagType::Close)
                }) {
                    if let Some(next) = node.next_sibling() {
                        if next.chunk().borrow().is_ws() {
                            detachables.push(next);
                        }
                    }
                } else if let Some(next) = child.next_sibling() {
                    if next.chunk().borrow().is_ws() {
                        detachables.push(next);
                    }
                }
            } else if child_chunk.is_tag() {
                let name = child_chunk.tag().unwrap().name();
                if matches!(name.as_str(), "p" | "ziyy" | "$root" | "div") {
                    if let Some(first) = child.first_child() {
                        if first.chunk().borrow().is_ws() {
                            detachables.push(first);
                        }
                    }
                } else if name == "br" {
                    if let Some(prev) = child.prev_sibling() {
                        if prev.chunk().borrow().is_ws() {
                            detachables.push(prev);
                        }
                    }

                    if let Some(next) = child.next_sibling() {
                        if next.chunk().borrow().is_ws() {
                            detachables.push(next);
                        }
                    }
                }
            }

            Resolver::optimize_ws(&child, detachables);
        }
    }

    fn _resolve(node: &Rc<Node>, node_name: &str) {
        for child in node.children() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_tag() {
                let tag = child_chunk.tag_mut().unwrap();
                if tag.r#type == TagType::Open {
                    let name = tag.name();
                    if matches!(name.as_str(), "ziyy" | "p" | "div") {
                        if matches!(node_name, "ziyy" | "$root" | "p" | "div")
                            && node
                                .first_child()
                                .is_some_and(|first| first.id() == child.id())
                        {
                        } else {
                            child.insert_before(Chunk {
                                data: ChunkData::WhiteSpace("\n".to_string()),
                                span: Span::inserted(),
                            });
                        }
                    } else if name == "a" {
                        for grand_child in child.children() {
                            grand_child.null_tags();
                        }
                    }

                    let last = child.last_child().unwrap();
                    let mut last_chunk = last.chunk().borrow_mut();
                    if last_chunk.is_tag_and(|tag| tag.r#type == TagType::Close) {
                        *last_chunk.tag_mut().unwrap() = !tag.clone();
                    } else {
                        last.insert_after(Chunk {
                            data: ChunkData::Tag(!tag.clone()),
                            span: Span::inserted(),
                        });
                    }
                }

                Resolver::_resolve(&child, tag.name());
            } else {
                Resolver::_resolve(&child, node_name);
            }
        }
    }
}
