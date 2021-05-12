use std::collections::LinkedList;
use std::rc::Rc;
use std::fmt::Debug;
use solver::model;


pub trait State : Eq + PartialEq + Sized + Clone {

}
pub trait Action : Eq + PartialEq + Sized + Clone {

}
pub trait Player: Eq + PartialEq + Sized + Clone {

}

pub trait Game <T : Eq + PartialEq + Sized + Clone + Debug, P: Player, B: board::Board<T>, A: Action> {
    fn board(&self) -> &B;
    fn actions(&self) -> LinkedList< A >;
    fn score(&self, player : &P) -> i64;
    fn is_terminal(&self) -> bool;
    fn apply(&self, action: &A) -> Self;
}

