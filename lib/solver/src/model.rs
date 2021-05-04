pub mod solver {
    use std::collections::LinkedList;
    use std::rc::Rc;

    pub trait Solvable {
        fn value(&self)->i64;
        fn next_states(&self)->LinkedList< Rc<Self> > where Self: Sized;
    }

}
