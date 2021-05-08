use std::collections::LinkedList;
use std::hash::Hash;
use std::rc::Rc;

pub trait Nodeable : Eq + PartialEq + Sized + Hash {
    fn nexts(&self) -> Vec< Rc<Self> >;
    fn distance(&self, target: &Self) -> i64;
}


#[derive(Hash, Clone, Debug)]
pub struct Way< T : Nodeable >{
    pub nodes : LinkedList< Rc<T> >,
    pub distance: i64,
}

