use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
    
    // return the element of stack top
    pub fn pop(&mut self) -> Option<T> {
        // match option {None => None, Some(x) => Some(y)} is what map() doing here
        // just no need to write None => None again
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // this while loop will continue as long as cur_link is Some
        // loop will break when cur_link is None
        while let Some(mut box_node) = cur_link {
            cur_link = mem::replace(&mut box_node.next, None);
        }
    }
}