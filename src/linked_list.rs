pub struct List<T> {
    head: Option<Node<T>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> List<T> {
    
    pub fn new() -> Self {
        List { head: None, len: 0 }
    }

    fn get_last(&mut self) -> Option<&mut Node<T>> {
        if let Some(ref mut node) = self.head {
            return Some(node.get_last());
        } else {
            None
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node { value, next: None };
        if let Some(node) = self.get_last() {
            node.get_last().next = Some(Box::new(new_node));
        } else {
            self.head = Some(new_node);
        }
        self.len += 1;
    }
}

impl <T> Node<T> {
    pub fn get_last(&mut self) -> &mut Self {
        if let Some(ref mut node) = self.next {
            return node.get_last();
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_empty() {
        const EXPECTED_LEN: usize = 0;
        let list: List<i32> = List::new();
        assert_eq!(EXPECTED_LEN, list.len);
    }

    #[test]
    fn test_len_1() {
        const EXPECTED_LEN: usize = 1;
        let mut list: List<i32> = List::new();
        list.push(1);
        assert_eq!(EXPECTED_LEN, list.len);
    }
}