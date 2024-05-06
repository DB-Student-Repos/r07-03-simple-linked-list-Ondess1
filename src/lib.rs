use std::iter::FromIterator;
pub struct SimpleLinkedList<T> {
    head: Option<Box<Element<T>>>,
    size: usize,
}

pub struct Element<T> {
    data: T,
    next: Option<Box<Element<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, data: T) {
        let new_element = Box::new(Element {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_element);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut element| {
            self.head = element.next.take();
            self.size -= 1;
            element.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|element| &element.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut prev = None;
        let mut current = self.head.take();
        while let Some(mut element) = current {
            let next = element.next.take();
            element.next = prev.take();
            prev = Some(element);
            current = next;
        }
        SimpleLinkedList {
            head: prev,
            size: self.size,
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(list.len());
        while let Some(item) = list.pop() {
            vec.push(item);
        }
        vec.reverse(); // Reverse the order to match the expected order of elements
        vec
    }
}
