struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            value: elem,
            next: None,
        }
    }
    
    fn set_next(&mut self, node: Self) {
        self.next = Some(Box::new(node));
    }
    
    fn get_last(&mut self) -> &mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        } 
        self
    }
    
    fn push(&mut self, elem: T) {
        let new_node = Node::new(elem);
        self.get_last().set_next(new_node);
    }
}