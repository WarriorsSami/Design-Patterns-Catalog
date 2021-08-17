use std::collections::BinaryHeap;
use std::option::Option;
use std::cmp::Ord;

pub trait AbstractIterator<T> {
    fn next(&mut self) -> Option<T>;
    fn has_more(&self) -> bool;
    fn get_current(&self) -> Option<T>;
}

pub struct VecIterator<T> {
    current: i32,
    vector: Vec<T>
}

impl<T: Clone> VecIterator<T> {
    pub(crate) fn new(vec: &Vec<T>) -> VecIterator<T> {
        VecIterator {
            current: -1,
            vector: vec.to_vec()
        }
    }
}

impl<T: Clone> AbstractIterator<T> for VecIterator<T> {
    fn next(&mut self) -> Option<T> {
        self.current += 1;
        self.vector.get(self.current as usize).cloned()
    }

    fn has_more(&self) -> bool {
        match self.get_current() {
            Some(_) => true,
            None => false
        }
    }

    fn get_current(&self) -> Option<T> {
        self.vector.get(self.current as usize).cloned()
    }
}

pub struct BHIterator<T> {
    current: Option<T>,
    heap: BinaryHeap<T>
}

impl<T: Clone> BHIterator<T> {
    pub(crate) fn new(heap: BinaryHeap<T>) -> BHIterator<T> {
        BHIterator {
            current: None,
            heap: heap.clone()
        }
    }
}

impl<T: Clone + Ord> AbstractIterator<T> for BHIterator<T> {
    fn next(&mut self) -> Option<T> {
        self.current = self.heap.pop();
        self.get_current()
    }

    fn has_more(&self) -> bool {
        match self.current {
            Some(_) => true,
            None => false
        }
    }

    fn get_current(&self) -> Option<T> {
        self.current.clone()
    }
}