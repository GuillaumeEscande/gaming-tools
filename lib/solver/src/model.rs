pub mod solver {
    use std::collections::LinkedList;

    pub trait Solvable : Eq + PartialEq + Sized {
        fn value(&self)->i64;
        fn next_states(&self)->LinkedList< Self >;
    }

}
