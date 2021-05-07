pub mod base {
    use std::collections::LinkedList;

    pub trait Action : Eq + PartialEq + Sized + Clone {

    }
    pub trait Game : <B: Board, A: Action> {
        fn state(&self) -> &B;
        fn actions(&self) -> LinkedList< A >;
    }

}