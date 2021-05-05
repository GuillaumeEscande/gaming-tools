pub mod base {
    use std::vec::Vec;

    pub trait BoardCase< T: Eq + PartialEq + Sized > {
        fn position(&self)->Vec<i64>;
        fn get_value(&self)->T;
        fn goto(&self, origin: &T, target: &T)->i64;
    }

    pub trait Board< T : Eq + PartialEq + Sized > {
        fn neighbors(&self, origin: &dyn BoardCase<T> )->Vec<&dyn BoardCase<T> >;
        fn sub_board(&self, origin: Vec<i64>, size: Vec<i64> )->dyn Board<T>;
        fn print(&self);
    }

    pub struct Board2D< T : Eq + PartialEq + Sized > {
        board : Vec< Vec<T> >
    }




}