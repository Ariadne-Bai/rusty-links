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
    
    // return a reference to the top element, or just return None if empty
    // so return type if Option<&T>
    // note that we are not taking the thing out from the option
    // and we are not moving option out of the thing it is in
    // we only need a reference to the thing inside the Option
    // impl<T> Option<T> {
    //     fn as_ref(&self) -> Option<&T>;
    // }
    pub fn peek(& self) -> Option<& T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
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

#[test]
fn peek() {
    let mut list = List::<i32>::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    list.peek_mut().map(|value| {
        *value = 42;
    });
    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
}