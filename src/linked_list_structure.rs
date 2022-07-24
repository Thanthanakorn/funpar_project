struct TTList<T> { //TT comes from Thana and Thanakorn
first: Option<Box<Node<T>>>,
}
impl<T> TTList<T> {
    pub fn new() -> Self {
        Self { first: None }
    }

    pub fn push(&mut self, element: T) {
        if let Some(node) = &mut self.first {
            node.push(element)
        } else {
            self.first = Some(Box::new(Node {
                e: element,
                next: None,
            }));
        }
    }
}

impl<T> IntoIterator for TTList<T> {
    type Item = T;
    type IntoIter = Listiterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        Listiterator { lst: self }
    }
}
struct Listiterator<T> {
    lst: TTList<T>,
}

impl<T> Iterator for Listiterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let temp = std::mem::replace(&mut self.lst.first, unsafe { std::mem::zeroed() });
        if let Some(node) = temp {
            let Node {
                next: nextnode,
                e: ans,
            } = *node;
            self.lst.first = nextnode;
            Some(ans)
        } else {
            None
        }
    }
}
struct Node<T> {
    e: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn push(&mut self, element: T) {
        if let Some(node) = &mut self.next {
            node.push(element)
        } else {
            self.next = Some(Box::new(Self {
                e: element,
                next: None,
            }));
        }
    }
}
fn main() {
    let mut l = TTList::new();
    list.push(1);
    for i in list {
        println!("{}", i);
    }
}
