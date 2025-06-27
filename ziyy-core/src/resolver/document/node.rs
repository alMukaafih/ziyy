use std::{
    cell::{Ref, RefCell},
    rc::{Rc, Weak},
};

use super::Document;
use crate::parser::chunk::Chunk;

#[derive(Debug, Clone)]
struct Kin {
    parent: Option<u32>,
    prev_sibling: Option<u32>,
    next_sibling: Option<u32>,
    children: Option<(u32, u32)>,
}

#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct Node {
    id: u32,
    kin: RefCell<Kin>,
    doc: Weak<Document>,
    chunk: RefCell<Chunk>,
}

impl Node {
    pub fn new(id: u32, chunk: Chunk, doc: Weak<Document>) -> Self {
        Self {
            id,
            kin: RefCell::new(Kin {
                parent: None,
                prev_sibling: None,
                next_sibling: None,
                children: None,
            }),
            doc,
            chunk: RefCell::new(chunk),
        }
    }

    pub fn id(self: &Rc<Node>) -> u32 {
        self.id
    }

    pub fn doc(self: &Rc<Node>) -> Rc<Document> {
        self.doc.upgrade().unwrap()
    }

    /// Returns the chunk of this node.
    pub fn chunk(self: &Rc<Node>) -> &RefCell<Chunk> {
        &self.chunk
    }

    fn axis<F>(self: &Rc<Node>, f: F) -> Option<Rc<Node>>
    where
        F: FnOnce(Ref<Kin>) -> Option<u32>,
    {
        f(self.kin.borrow()).map(|id| self.doc().get(id))
    }

    /// Returns the parent of this node.
    pub fn parent(self: &Rc<Node>) -> Option<Rc<Self>> {
        self.axis(|node| node.parent)
    }

    /// Returns the previous sibling of this node.
    pub fn prev_sibling(self: &Rc<Node>) -> Option<Rc<Self>> {
        self.axis(|node| node.prev_sibling)
    }

    /// Returns the next sibling of this node.
    pub fn next_sibling(self: &Rc<Node>) -> Option<Rc<Self>> {
        self.axis(|node| node.next_sibling)
    }

    /// Returns the first child of this node.
    pub fn first_child(self: &Rc<Node>) -> Option<Rc<Self>> {
        self.axis(|node| node.children.map(|(id, _)| id))
    }

    /// Returns the last child of this node.
    pub fn last_child(self: &Rc<Node>) -> Option<Rc<Self>> {
        self.axis(|node| node.children.map(|(_, id)| id))
    }

    /// Returns true if this node has children.
    pub fn has_children(self: &Rc<Node>) -> bool {
        self.kin.borrow().children.is_some()
    }

    /// Appends a new child to this node.
    pub fn append(self: &Rc<Node>, value: Chunk) -> Rc<Node> {
        let id = self.doc().orphan(value).id;
        self.append_id(id)
    }

    /// Prepends a new child to this node.
    pub fn prepend(self: &Rc<Node>, value: Chunk) -> Rc<Node> {
        let id = self.doc().orphan(value).id;
        self.prepend_id(id)
    }

    /// Inserts a new sibling before this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_before(self: &Rc<Node>, value: Chunk) -> Rc<Node> {
        let id = self.doc().orphan(value).id;
        self.insert_id_before(id)
    }

    /// Inserts a new sibling after this node.
    ///
    /// # Panics
    ///
    /// Panics if this node is an orphan.
    pub fn insert_after(self: &Rc<Node>, value: Chunk) -> Rc<Node> {
        let id = self.doc().orphan(value).id;
        self.insert_id_after(id)
    }

    /// Detaches this node from its parent.
    pub fn detach(self: &Rc<Node>) {
        let mut kin = self.kin.borrow_mut();
        let parent_id = match kin.parent {
            Some(id) => id,
            None => return,
        };
        let prev_sibling_id = kin.prev_sibling;
        let next_sibling_id = kin.next_sibling;

        {
            kin.parent = None;
            kin.prev_sibling = None;
            kin.next_sibling = None;
        }

        if let Some(id) = prev_sibling_id {
            self.doc().node(id).kin.borrow_mut().next_sibling = next_sibling_id;
        }
        if let Some(id) = next_sibling_id {
            self.doc().node(id).kin.borrow_mut().prev_sibling = prev_sibling_id;
        }

        let doc = self.doc();
        let parent = doc.node(parent_id);
        let mut parent_kin = parent.kin.borrow_mut();
        let (first_child_id, last_child_id) = parent_kin.children.unwrap();
        if first_child_id == last_child_id {
            parent_kin.children = None;
        } else if first_child_id == self.id {
            parent_kin.children = Some((next_sibling_id.unwrap(), last_child_id));
        } else if last_child_id == self.id {
            parent_kin.children = Some((first_child_id, prev_sibling_id.unwrap()));
        }
    }

    /// Appends a child to this node.
    pub fn append_id(self: &Rc<Node>, new_child_id: u32) -> Rc<Node> {
        assert_ne!(
            self.id, new_child_id,
            "Cannot append node as a child to itself"
        );

        let mut kin = self.kin.borrow_mut();

        let last_child_id = kin.children.map(|(_, id)| id);

        if last_child_id != Some(new_child_id) {
            {
                let new_child = self.doc().get(new_child_id);
                new_child.detach();
                let mut new_child_kin = new_child.kin.borrow_mut();
                new_child_kin.parent = Some(self.id);
                new_child_kin.prev_sibling = last_child_id;
            }

            if let Some(id) = last_child_id {
                self.doc().node(id).kin.borrow_mut().next_sibling = Some(new_child_id);
            }

            kin.children = match kin.children {
                Some((first_child_id, _)) => Some((first_child_id, new_child_id)),
                None => Some((new_child_id, new_child_id)),
            };
        }

        self.doc().get(new_child_id)
    }

    /// Prepends a child to this node.
    pub fn prepend_id(self: &Rc<Node>, new_child_id: u32) -> Rc<Node> {
        assert_ne!(
            self.id, new_child_id,
            "Cannot prepend node as a child to itself"
        );

        let mut kin = self.kin.borrow_mut();

        let first_child_id = kin.children.map(|(id, _)| id);

        if first_child_id != Some(new_child_id) {
            let new_child = self.doc().get(new_child_id);
            new_child.detach();
            let mut new_child_kin = new_child.kin.borrow_mut();
            new_child_kin.parent = Some(self.id);
            new_child_kin.next_sibling = first_child_id;

            if let Some(id) = first_child_id {
                self.doc().node(id).kin.borrow_mut().prev_sibling = Some(new_child_id);
            }

            kin.children = match kin.children {
                Some((_, last_child_id)) => Some((new_child_id, last_child_id)),
                None => Some((new_child_id, new_child_id)),
            };
        }

        self.doc().get(new_child_id)
    }

    /// Inserts a sibling before this node.
    ///
    /// # Panics
    ///
    /// - Panics if `new_sibling_id` is not valid.
    /// - Panics if this node is an orphan.
    pub fn insert_id_before(self: &Rc<Node>, new_sibling_id: u32) -> Rc<Node> {
        assert_ne!(
            self.id, new_sibling_id,
            "Cannot insert node as a sibling of itself"
        );

        let mut kin = self.kin.borrow_mut();

        let parent_id = kin.parent.unwrap();
        let prev_sibling_id = kin.prev_sibling;

        {
            let new_sibling = self.doc().get(new_sibling_id);
            new_sibling.detach();
            let mut new_sibling_kin = new_sibling.kin.borrow_mut();
            new_sibling_kin.parent = Some(parent_id);
            new_sibling_kin.prev_sibling = prev_sibling_id;
            new_sibling_kin.next_sibling = Some(self.id);
        }

        if let Some(id) = prev_sibling_id {
            self.doc().node(id).kin.borrow_mut().next_sibling = Some(new_sibling_id);
        }

        kin.prev_sibling = Some(new_sibling_id);

        {
            let doc = self.doc();
            let parent = doc.node(parent_id);
            let mut parent_kin = parent.kin.borrow_mut();
            let (first_child_id, last_child_id) = parent_kin.children.unwrap();
            if first_child_id == self.id {
                parent_kin.children = Some((new_sibling_id, last_child_id));
            }
        }

        self.doc().get(new_sibling_id)
    }

    /// Inserts a sibling after this node.
    ///
    /// # Panics
    ///
    /// - Panics if `new_sibling_id` is not valid.
    /// - Panics if this node is an orphan.
    pub fn insert_id_after(self: &Rc<Node>, new_sibling_id: u32) -> Rc<Node> {
        assert_ne!(
            self.id, new_sibling_id,
            "Cannot insert node as a sibling of itself"
        );

        let mut kin = self.kin.borrow_mut();

        let parent_id = kin.parent.unwrap();
        let next_sibling_id = kin.next_sibling;

        {
            let new_sibling = self.doc().get(new_sibling_id);
            new_sibling.detach();
            let mut new_sibling_kin = new_sibling.kin.borrow_mut();
            new_sibling_kin.parent = Some(parent_id);
            new_sibling_kin.prev_sibling = Some(self.id);
            new_sibling_kin.next_sibling = next_sibling_id;
        }

        if let Some(id) = next_sibling_id {
            self.doc().node(id).kin.borrow_mut().prev_sibling = Some(new_sibling_id);
        }

        kin.next_sibling = Some(new_sibling_id);

        {
            let doc = self.doc();
            let parent = doc.node(parent_id);
            let mut parent_kin = parent.kin.borrow_mut();
            let (first_child_id, last_child_id) = parent_kin.children.unwrap();
            if last_child_id == self.id {
                parent_kin.children = Some((first_child_id, new_sibling_id));
            }
        }

        self.doc().get(new_sibling_id)
    }

    /// Returns the string representation of this node.
    pub fn to_string(self: &Rc<Node>, buf: &mut String) {
        if self.has_children() {
            let tag_chunk = self.chunk.borrow();
            let tag = tag_chunk.data.tag().unwrap();
            buf.push_str(tag.to_string().as_str());
            for child in self.children() {
                child.to_string(buf);
            }
            //buf.push_str((!tag.clone()).to_string().as_str());
        } else {
            buf.push_str(self.chunk.borrow().data.to_string().as_str());
        }
    }

    pub fn strip_styles(self: &Rc<Node>) {
        if self.chunk.borrow().is_tag() {
            let mut tag_chunk = self.chunk.borrow_mut();
            let tag = tag_chunk.data.tag_mut().unwrap();
            tag.reset_styles();
            for child in self.children() {
                child.strip_styles();
            }
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        let kin = self.kin.borrow();
        let other_kin = other.kin.borrow();
        self.id == other.id
            && kin.parent == other_kin.parent
            && kin.prev_sibling == other_kin.prev_sibling
            && kin.next_sibling == other_kin.next_sibling
            && kin.children == other_kin.children
            && self.chunk == other.chunk
    }
}

impl Eq for Node {}
