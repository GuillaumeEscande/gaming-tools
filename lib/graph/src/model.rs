use std::collections::LinkedList;
use std::hash::Hash;
use std::rc::Rc;


#[derive(Hash, Clone, Debug)]
pub struct Way< T : Nodeable >{
    pub nodes : LinkedList< Rc<BoardCase> >,
    pub distance: i64,
}

