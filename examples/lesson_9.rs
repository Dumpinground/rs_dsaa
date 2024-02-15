// Own Memory on the Heap with Box, String and Vecs

use std::ops::AddAssign;

#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };

    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }

    let mut v = Vec::with_capacity(100);
    v.push("hello".to_owned());
    v.push("goodbye".into());

    for i in 0..105 {
        v.push(i.to_string());
    }

    println!("v.len = {}, capacity = {}", v.len(), v.capacity());

    println!("Hello, {:?}", ll);
}
