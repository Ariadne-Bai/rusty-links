
// this is an enum! it's either empty or some list`
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }
use std::mem;

pub struct List {
    head: Link,
}
struct Node {
    elem: i32,
    next: Link,   // next is either a pointer of Empty(rust has no null pointer)
}

enum Link {
    Empty,
    More(Box<Node>),    // we are not allowed to publicly talk about private types
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
}