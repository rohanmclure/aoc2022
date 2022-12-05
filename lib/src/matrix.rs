use std::alloc::{Allocator, System, Layout};
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;

#[derive(Clone)]
pub struct Matrix<T> {
    dims: (usize, usize),
    layout: Layout,
    base_ptr: NonNull<T>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(m: usize, n: usize) -> Self {
        let layout = Layout::array::<T>(m*n).unwrap();
        Matrix { dims: (m, n),
                 layout: layout,
                 base_ptr: System.allocate(layout)
                                 .unwrap().cast::<T>() }
    }

    pub fn get_dims(&self) -> (usize, usize) {
        self.dims
    }

    pub fn fill(&mut self, e: T) {
        let (m, n) = self.dims;
        for j in 0..n {
            for i in 0..m {
                self[(i, j)] = e;
            }
        }
    }
}

impl<T> Drop for Matrix<T> {
    fn drop(&mut self) {
        unsafe {
            System.deallocate(self.base_ptr.cast::<u8>(), self.layout);
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (m, _) = self.dims;
        let (i, j) = index;
        unsafe {
            self.base_ptr.as_ptr().add(i + j*m).as_ref().unwrap()
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (m, _) = self.dims;
        let (i, j) = index;
        unsafe {
            self.base_ptr.as_ptr().add(i + j*m).as_mut().unwrap()
        }
    }
}