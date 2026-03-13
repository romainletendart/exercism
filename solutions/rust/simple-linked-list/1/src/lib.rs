#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Self>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    _len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            _len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self._len
    }

    pub fn push(&mut self, element: T) {
        let new_head = Box::new(Node {
            value: element,
            next: self.head.take(),
        });
        self.head = Some(new_head);
        self._len += 1
    }

    pub fn pop(&mut self) -> Option<T> {
        let current_head = self.head.take();
        if let Some(node) = current_head {
            let new_head = node.next;
            let element = node.value;
            self.head = new_head;
            self._len -= 1;
            return Some(element);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|inner| &inner.value)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed_list = Self::new();
        let mut head = self.head;
        while let Some(node) = head {
            reversed_list.push(node.value);
            head = node.next;
        }
        reversed_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut linked_list = Self::new();
        iter.into_iter()
            .for_each(|element| linked_list.push(element));
        linked_list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(linked_list.len());
        while let Some(element) = linked_list.pop() {
            vec.push(element);
        }
        vec.reverse();
        vec
    }
}
