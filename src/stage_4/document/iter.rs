use std::rc::Rc;

use super::Node;

#[derive(Debug)]
pub struct Children {
    front: Option<Rc<Node>>,
    back: Option<Rc<Node>>,
}

impl Clone for Children {
    fn clone(&self) -> Self {
        Self {
            front: self.front.clone(),
            back: self.back.clone(),
        }
    }
}

impl Iterator for Children {
    type Item = Rc<Node>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            let node = self.front.take();
            self.back = None;
            node
        } else {
            let node = self.front.take();
            self.front = node.as_ref().and_then(Node::next_sibling);
            node
        }
    }
}

/// Open or close edge of a node.
#[derive(Debug, Clone)]
pub enum Edge {
    /// Open.
    Open(Rc<Node>),
    /// Close.
    Close(Rc<Node>),
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Edge::Open(a), Edge::Open(b)) | (Edge::Close(a), Edge::Close(b)) => a == b,
            _ => false,
        }
    }
}

/// Iterator which traverses a subtree.
#[derive(Debug, Clone)]
pub struct Traverse {
    root: Option<Rc<Node>>,
    edge: Option<Edge>,
}

impl Iterator for Traverse {
    type Item = Edge;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.edge {
            None => {
                if let Some(root) = &self.root {
                    self.edge = Some(Edge::Open(root.clone()));
                }
            }
            Some(Edge::Open(node)) => {
                if let Some(first_child) = node.first_child() {
                    self.edge = Some(Edge::Open(first_child));
                } else {
                    self.edge = Some(Edge::Close(node.clone()));
                }
            }
            Some(Edge::Close(node)) => {
                if *node == self.root.clone().unwrap() {
                    self.root = None;
                    self.edge = None;
                } else if let Some(next_sibling) = node.next_sibling() {
                    self.edge = Some(Edge::Open(next_sibling));
                } else {
                    self.edge = node.parent().map(Edge::Close);
                }
            }
        }
        self.edge.clone()
    }
}

/// Iterator over a node and its descendants.
#[derive(Debug)]
pub struct Descendants(Traverse);

impl Clone for Descendants {
    fn clone(&self) -> Self {
        Descendants(self.0.clone())
    }
}

impl Iterator for Descendants {
    type Item = Rc<Node>;
    fn next(&mut self) -> Option<Self::Item> {
        for edge in &mut self.0 {
            if let Edge::Open(node) = edge {
                return Some(node);
            }
        }
        None
    }
}

macro_rules! axis_iterators {
    ($(#[$m:meta] $i:ident($f:path);)*) => {
        $(
            #[$m]
            #[derive(Debug)]
            pub struct $i(Option<Rc<Node>>);
            impl Clone for $i {
                fn clone(&self) -> Self {
                    $i(self.0.clone())
                }
            }

            impl Iterator for $i {
                type Item = Rc<Node>;
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().and_then($f);
                    node
                }
            }
        )*
    };
}

axis_iterators! {
    /// Iterator over ancestors.
    Ancestors(Node::parent);

    /// Iterator over previous siblings.
    PrevSiblings(Node::prev_sibling);

    /// Iterator over next siblings.
    NextSiblings(Node::next_sibling);

    /// Iterator over first children.
    FirstChildren(Node::first_child);

    /// Iterator over last children.
    LastChildren(Node::last_child);
}

impl Node {
    /// Returns an iterator over children.
    pub fn children(self: &Rc<Node>) -> Children {
        Children {
            front: self.first_child(),
            back: self.last_child(),
        }
    }

    /// Returns an iterator which traverses the subtree starting at this node.
    pub fn traverse(self: &Rc<Node>) -> Traverse {
        Traverse {
            root: Some(self.clone()),
            edge: None,
        }
    }

    /// Returns an iterator over this node and its descendants.
    pub fn descendants(self: &Rc<Node>) -> Descendants {
        Descendants(self.traverse())
    }

    /// Returns an iterator over ancestors.
    pub fn ancestors(self: &Rc<Node>) -> Ancestors {
        Ancestors(Some(self.clone()))
    }
}
