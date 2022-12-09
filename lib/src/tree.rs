pub struct Tree<T> {
    nodes: Vec<Box<Tree<T>>>,
    elem: T
}

impl<T> Tree<T> {
    pub fn new(e: T) -> Self {
        Tree { nodes: vec![], elem: e }
    }

    pub fn add_subtree(&mut self, t: Tree<T>) {
        self.nodes.push(Box::new(t))
    }

    pub fn branches_iter(&self) -> impl Iterator<Item=&Tree<T>> {
        self.nodes.iter().map(|s| &**s)
    }

    pub fn branches_iter_mut(&mut self)
        -> impl Iterator<Item=&mut Tree<T>> {
        self.nodes.iter_mut().map(|s| &mut **s)
    }
}

/*
impl<T> Drop for Tree<T> {
    fn drop(&mut self) {
        for n in self.nodes.iter_mut() {
            drop(n);
        }
    }
}
*/