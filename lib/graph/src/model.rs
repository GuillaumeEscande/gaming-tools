use std::collections::LinkedList;
use std::hash::Hash;
use std::rc::Rc;
use board::BoardCase;
use std::fmt::Debug;


#[derive(Debug)]
pub struct Way< T : Eq + PartialEq + Sized + Hash + Debug >{
    pub nodes : LinkedList< Rc< dyn BoardCase<T>> >,
    pub distance: i16,
}

