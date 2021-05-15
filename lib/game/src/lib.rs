use std::collections::LinkedList;
use std::rc::Rc;
use std::fmt::Debug;
use board::Board;
use board::BoardCase;


pub trait Game < T : Eq + PartialEq + Sized + Clone + Debug,
                 C : BoardCase<T>, 
                 P: Eq + PartialEq + Sized + Clone + Debug,
                 B: Board<T, C >,
                 A: Eq + PartialEq + Sized + Clone + Debug> : Eq + PartialEq + Sized + Clone + Debug {
    fn board(&self) -> Rc<B>;
    fn actions(&self) -> LinkedList< A >;
    fn score(&self) -> i64;
    fn me(&self) -> &P;
    fn is_terminal(&self) -> bool;
    fn apply(&self, action: &A) -> Rc<Self>;
}

