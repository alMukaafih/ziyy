use std::{collections::HashMap, rc::Rc};

use crate::{
    BUILTIN_TAGS,
    common::Span,
    parser::{
        chunk::{Chunk, ChunkData},
        tag_parer::tag::{Tag, TagType},
        word_parer::ansi::Ansi,
    },
};
use document::{Document, Node};

pub mod document;

#[doc(hidden)]
pub struct Resolver {
    ansi_only: bool,
}

impl Resolver {
    pub fn new(ansi_only: bool) -> Self {
        Self { ansi_only }
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
            }
        }

        let node = tree.root();

        if self.ansi_only {
            Resolver::optimize_ansi(&node);
            Resolver::constrain(&node);
            return tree;
        }

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
        Resolver::optimize_styles(&node);
        Resolver::constrain(&node);

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
                } else if name == "pre" {
                    continue;
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
                    if matches!(name.as_str(), "ziyy" | "p" | "div" | "pre") {
                        if matches!(node_name, "ziyy" | "$root" | "p" | "div" | "pre")
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
                            grand_child.strip_styles();
                        }
                    }

                    let last = child.last_child().unwrap();
                    let last_chunk = last.chunk().borrow_mut();
                    if !last_chunk.is_tag_and(|tag| tag.r#type == TagType::Close) {
                        last.insert_after(Chunk {
                            data: ChunkData::Tag(tag.close()),
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

    /// Optimize styles
    fn optimize_styles(node: &Rc<Node>) {
        let mut stack = Vec::with_capacity(1024);
        for child in node.descendants() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_tag() {
                let tag = child_chunk.tag_mut().unwrap();
                match tag.r#type {
                    TagType::Open => {
                        let (prev_name, prev_style, prev_delta): (String, Ansi, Ansi) =
                            stack.pop().unwrap_or_default();
                        let new_style = prev_style.clone() + tag.clone().ansi;
                        let new_delta = tag.clone().ansi - prev_style.clone();

                        tag.ansi = new_delta.clone();

                        stack.push((prev_name, prev_style, prev_delta));
                        stack.push((tag.name().clone(), new_style, new_delta));
                    }

                    TagType::Close => {
                        let mut current_tag = stack.pop().unwrap_or_default();
                        let new_tag = current_tag.clone();
                        while current_tag.0 == "$ansi" {
                            current_tag = stack.pop().unwrap_or_default();
                        }

                        tag.ansi = !(new_tag.2);
                    }

                    TagType::SelfClose => {}
                }
            }
        }
    }

    fn constrain(node: &Rc<Node>) {
        if let Some(first) = node.first_child() {
            first.insert_before(Chunk {
                data: ChunkData::Word(String::from("\x1b[m")),
                span: Span::inserted(),
            });
        }

        if let Some(last) = node.last_child() {
            last.insert_after(Chunk {
                data: ChunkData::Word(String::from("\x1b[m")),
                span: Span::inserted(),
            });
        }
    }

    fn optimize_ansi(node: &Rc<Node>) {
        for child in node.children() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_tag() {
                if let Some(first) = child.first_child() {
                    let mut first_chunk = first.chunk().borrow_mut();
                    if first_chunk.is_tag() {
                        let tag = child_chunk.tag_mut().unwrap();
                        let first_tag = first_chunk.tag_mut().unwrap();

                        tag.reset_styles();
                        *first_tag = tag.clone() + first_tag.clone();
                    }
                }
            }
            Resolver::optimize_ansi(&child);
        }
    }
}
