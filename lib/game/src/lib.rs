use std::collections::LinkedList;

pub trait State : Eq + PartialEq + Sized + Clone {

}
pub trait Action : Eq + PartialEq + Sized + Clone {

}
pub trait Player: Eq + PartialEq + Sized + Clone {

}

pub trait Game <T : Eq + PartialEq + Sized + Clone, P: Player, B: board::Board<T>, A: Action> {
    fn state(&self) -> &B;
    fn actions(&self) -> LinkedList< A >;
    fn score(&self, player : &P) -> i64;
    fn is_terminate(&self) -> bool;
    fn apply(&self, action: &A) -> Self;
}

