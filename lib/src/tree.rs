use std::ptr::NonNull;

pub trait Tree<T> {
    fn add_subtree(&mut self, t: Self);
    fn branches_iter(&self) -> Box::<dyn Iterator<Item=&Self> + '_>;
    fn branches_iter_mut(&mut self) -> Box::<dyn Iterator<Item=&mut Self> + '_>;

    fn get_elem(&self) -> &T;
    fn get_elem_mut(&mut self) -> &mut T;
}

pub struct BackTree<T> {
    nodes: Vec<*mut Self>,
    elem: T,
    backlink: Option<NonNull<Self>>
}

impl<T> BackTree<T> {
    pub fn new(e: T) -> Self {
        BackTree { nodes: vec![],
                   elem: e,
                   backlink: None
                 }
    }

    pub fn back_ref(&self) -> Option<&Self> {
        unsafe {
            self.backlink.map(|nn| nn.as_ref())
        }
    }

    pub fn back_ref_mut(&self) -> Option<&mut Self> {
        unsafe {
            self.backlink.map(|mut nn| nn.as_mut())
        }
    }
}

impl<T> Tree<T> for BackTree<T> {
    /* Needs to take ownership of the new subtree */
    fn add_subtree(&mut self, t: BackTree<T>) {
        unsafe {
            let ptr = Box::into_raw(Box::new(t));
            self.nodes.push(ptr);
                (*ptr).backlink = Some(NonNull::new_unchecked(self as *mut Self));
        }
    }

    fn branches_iter(&self) -> Box::<dyn Iterator<Item=&Self> + '_> {
        Box::new({
            self.nodes.iter().map(
                |s| unsafe { (*s).as_ref().unwrap() }
            )
        })
    }

    fn branches_iter_mut(&mut self) -> Box::<dyn Iterator<Item=&mut Self> + '_> {
        Box::new({
            self.nodes.iter_mut().map(
                |s| unsafe { (*s).as_mut().unwrap() }
            )
        })
    }


    fn get_elem(&self) -> &T {
        &self.elem
    }

    fn get_elem_mut(&mut self) -> &mut T {
        &mut self.elem
    }
}

/* post-order traversal */
impl<T> Drop for BackTree<T> {
    fn drop(&mut self) {

        unsafe {
            for n in self.nodes.iter_mut() {
                drop(&mut **n);
            }
            drop(&mut self.elem);
        }
    }
}