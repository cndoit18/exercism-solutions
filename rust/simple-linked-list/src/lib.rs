pub struct SimpleLinkedList<T> {
    head: Link<T>,
    size: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            elem: element,
            next: self.head.take(),
        }));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.size -= 1;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut r = SimpleLinkedList::new();
        while let Some(node) = self.pop() {
            r.push(node);
        }
        r
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .fold(SimpleLinkedList::new(), |mut r, elem| {
                r.push(elem);
                r
            })
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut r = Vec::new();
        while let Some(node) = linked_list.pop() {
            r.insert(0, node)
        }
        r
    }
}
