#[test]
fn test_add_and_remove() {
    let mut list = List::new();
    list.add(&1);
    list.pop();

    assert_eq!(list.pop(), None);
}

struct List<'a, T> {
    head: Link<'a, T>,
}

enum Link<'a, T> {
    Empty,
    More(Box<Node<'a, T>>),
}

struct Node<'a, T> {
    elem: &'a T,
    next: Link<'a, T>,
}

impl<'a, T> List<'a, T> {
    fn new() -> Self {
        Self { head: Link::Empty }
    }

    fn add(&mut self, data: &'a T) {
        self.head = Link::More(Box::new(Node {
            elem: data,
            next: Link::Empty,
        }));
    }

    fn pop(&mut self) -> Option<&'a T> {
        match self.head {
            Link::Empty => None,
            Link::More(first) => match first.next {
                Link::Empty => {
                    let e = first.elem;
                    self.head = Link::Empty;
                    Some(e)
                }
                Link::More(second) => second.pop(),
            },
        }
    }
}
