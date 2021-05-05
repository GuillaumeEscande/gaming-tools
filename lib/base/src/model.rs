pub mod base {
    use std::collections::LinkedList;

    pub trait Gameable : Eq + PartialEq + Sized + Clone {
        fn actions(self) -> LinkedList< (Self, i64) >;
    }

}