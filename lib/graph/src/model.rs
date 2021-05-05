pub mod graph {
    use std::collections::LinkedList;
    use std::hash::Hash;

    pub trait Nodeable : Eq + PartialEq + Sized + Hash {
        fn nexts(&self) -> LinkedList< Self >;
        fn distance(&self, &Self) -> i64;
    }
    

    #[derive(Hash, Clone)]
    pub struct Way< T : Nodeable >{
        nodes : LinkedList< T >,
        distance: i64,
    }
    
}