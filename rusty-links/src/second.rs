use std::mem;

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
    
    // return the element of stack top
    pub fn pop(&mut self) -> Option<i32> {
        // match option {None => None, Some(x) => Some(y)} is what map() doing here
        // just no need to write None => None again
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // this while loop will continue as long as cur_link is Some
        // loop will break when cur_link is None
        while let Some(mut box_node) = cur_link {
            cur_link = mem::replace(&mut box_node.next, None);
        }
    }
}