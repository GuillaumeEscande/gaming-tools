
use std::collections::LinkedList;
use std::rc::Rc;

pub trait Solvable : Eq + PartialEq + Sized {
    fn value(&self)->i64;
    fn next_states(&self)->LinkedList< Rc<Self> >;
}
