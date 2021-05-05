pub mod graph {
    use std::collections::LinkedList;
    use std::hash::Hash;

    pub trait Nodeable : Eq + PartialEq + Sized + Hash {
        fn nexts(&self) -> Vec< Self >;
        fn distance(&self, target: &Self) -> i64;
    }
    

    #[derive(Hash, Clone, Debug)]
    pub struct Way< T : Nodeable >{
        pub nodes : LinkedList< T >,
        pub distance: i64,
    }
    
}