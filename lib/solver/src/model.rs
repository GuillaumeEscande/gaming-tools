pub mod solver {
    use std::collections::LinkedList;

    pub trait Solvable {
        fn value(&self)->f64;
        fn next_states(&self)->LinkedList< Self > where Self: Sized;
    }

}