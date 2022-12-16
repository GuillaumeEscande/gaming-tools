use std::rc::Rc;
use std::fmt::Debug;
use board::Board;
use board::BoardCase;
use std::vec::Vec;


pub trait Game < T : Eq + PartialEq + Sized + Clone + Debug,
                 C : BoardCase<T>, 
                 P: Eq + PartialEq + Sized + Clone + Debug,
                 B: Board<T, C >,
                 A: Eq + PartialEq + Sized + Clone + Debug> : Eq + PartialEq + Sized + Clone + Debug {
    fn board(&self) -> &B;
    fn list_actions(&self) -> Vec< A >;
    fn apply_action(&self, action: &A) -> Rc<Self>;
    fn score(&self) -> i64;
    fn me(&self) -> &P;
    fn opposite(&self) -> Rc<Self>;
    fn is_terminal(&self) -> bool;
}




pub trait Strategy <    T : Eq + PartialEq + Sized + Clone + Debug,
                        C : BoardCase<T>, 
                        P: Eq + PartialEq + Sized + Clone + Debug,
                        B: Board<T, C >,
                        A: Eq + PartialEq + Sized + Clone + Debug,
                        G : Game<T, C, P, B, A> > : Eq + PartialEq + Sized + Clone + Debug {
    fn compute_action(&self, game: &G) -> A;
}