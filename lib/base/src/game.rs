pub mod base {
    use std::collections::LinkedList;
    use crate::board::base;

    pub trait State : Eq + PartialEq + Sized + Clone {

    }
    pub trait Action : Eq + PartialEq + Sized + Clone {

    }
    pub trait Player: Eq + PartialEq + Sized + Clone {
    }
    pub trait Game <T : Eq + PartialEq + Sized + Clone, P: Player, B: base::Board<T>, A: Action> {
        fn state(&self) -> &B;
        fn actions(&self) -> LinkedList< A >;
        fn score(&self, player : &P) -> i64;
        fn is_terminate(&self) -> bool;
        fn apply(&self, action: &A) -> Self;
    }

}