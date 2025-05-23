use std::{collections::HashMap, rc::Rc};

use crate::stage_3::{
    chunk::Chunk,
    tag_parer::tag::{Tag, TagType},
};
use document::{Document, Node};

pub mod document;

static BUILTIN_TAGS: &[&str] = &[
    "a", "b", "br", "d", "h", "i", "k", "p", "r", "s", "u", "ziyy",
];

pub struct Stage4 {
    bindings: HashMap<String, Tag>,
}

impl Stage4 {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn parse(&mut self, chunks: Vec<Chunk>) -> Rc<Document> {
        let tree = Document::new();
        let mut node = tree.root();

        for chunk in chunks.iter() {
            match chunk {
                chunk @ Chunk::Tag(tag) => match tag.r#type {
                    TagType::Open => {
                        node = node.append(chunk.clone());
                    }

                    TagType::Close => {
                        node = node.parent().unwrap();
                    }

                    TagType::SelfClose => {
                        node.append(chunk.clone());
                    }
                },
                ws @ Chunk::WhiteSpace(_) => {
                    node.append(ws.clone());
                }
                word @ Chunk::Word(_) => {
                    node.append(word.clone());
                }
            }
        }

        let node = tree.root();
        let mut detachables = vec![];
        self.collect_bindings(&node, &mut detachables);
        for node in &detachables {
            node.detach();
        }

        for _ in 0..3 {
            let mut detachables = vec![];
            self.optimize(&node, &mut detachables);
            for node in &detachables {
                node.detach();
            }
        }
        self.resolve(&node, "$root");

        tree
    }

    fn collect_bindings(&mut self, node: &Rc<Node>, detachables: &mut Vec<Rc<Node>>) {
        for child in node.children() {
            let child_chunk = child.chunk().borrow();
            if child_chunk.is_tag() {
                let name = child_chunk.tag().unwrap().name();
                if name == "let" {
                    let name = child_chunk.tag().unwrap().custom();
                    let id = node.id();
                    self.bindings.insert(
                        format!("{id}/{name}"),
                        child.chunk().borrow().tag().unwrap().clone(),
                    );
                    detachables.push(child.clone());
                }
            }
            self.collect_bindings(&child, detachables);
        }
    }

    fn optimize(&self, node: &Rc<Node>, detachables: &mut Vec<Rc<Node>>) {
        for child in node.children() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_ws() {
                if child.id() == 1 {
                    detachables.push(child.clone());
                }

                if child.id() as usize == child.doc().len() - 1
                    && child_chunk.ws().unwrap().contains("\n")
                {
                    *child_chunk = Chunk::WhiteSpace("\n".to_string());
                } else {
                    *child_chunk = Chunk::WhiteSpace(" ".to_string());
                }

                if let Some(Some(first)) = child.next_sibling().map(|next| next.first_child()) {
                    if first.chunk().borrow().is_ws() {
                        detachables.push(first.clone());
                    }
                } else if child.next_sibling().is_none() {
                    if let Some(next) = node.next_sibling() {
                        if next.chunk().borrow().is_ws() {
                            detachables.push(next.clone());
                        }
                    }
                } else if let Some(next) = child.next_sibling() {
                    if next.chunk().borrow().is_ws() {
                        detachables.push(next.clone());
                    }
                }
            } else if child_chunk.is_tag() {
                let name = child_chunk.tag().unwrap().name();
                if matches!(name.as_str(), "p" | "ziyy") {
                    if let Some(first) = child.first_child() {
                        if first.chunk().borrow().is_ws() {
                            detachables.push(first.clone());
                        }
                    }
                } else if name == "br" {
                    if let Some(prev) = child.prev_sibling() {
                        if prev.chunk().borrow().is_ws() {
                            detachables.push(prev.clone());
                        }
                    }

                    if let Some(next) = child.next_sibling() {
                        if next.chunk().borrow().is_ws() {
                            detachables.push(next.clone());
                        }
                    }
                }
            }

            self.optimize(&child, detachables);
        }
    }

    fn resolve(&self, node: &Rc<Node>, node_name: &str) {
        for child in node.children() {
            let mut child_chunk = child.chunk().borrow_mut();
            if child_chunk.is_tag() {
                let tag = child_chunk.tag_mut().unwrap();
                let name = tag.name();
                if name == "p" {
                    if matches!(node_name, "ziyy" | "$root" | "p")
                        && node.first_child().unwrap().id() == child.id()
                    {
                    } else {
                        child.insert_before(Chunk::WhiteSpace("\n".to_string()));
                    }
                }

                if !BUILTIN_TAGS.contains(&name.as_str()) {
                    for ansector in child.ancestors() {
                        if let Some(binding) =
                            self.bindings.get(&format!("{}/{}", ansector.id(), name))
                        {
                            tag.inherit(binding);
                            break;
                        }
                    }
                }

                if !tag.src().is_empty() {
                    for ansector in child.ancestors() {
                        if let Some(binding) =
                            self.bindings
                                .get(&format!("{}/{}", ansector.id(), tag.src()))
                        {
                            tag.inherit(binding);
                            break;
                        }
                    }
                }
                self.resolve(&child, &tag.name());
            } else {
                self.resolve(&child, node_name);
            }
        }
    }
}
