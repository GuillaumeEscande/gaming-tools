pub mod base {
    use std::vec::Vec;

    pub trait BoardCase< T: Eq + PartialEq + Sized > {
        fn position(&self)->Vec<i64>;
        fn get_value(&self)->T;
    }

    pub trait Board< T : Eq + PartialEq + Sized > {
        fn neighbors(&self, origin: &dyn BoardCase<T> )->Vec<&dyn BoardCase<T> >;
        fn sub_board(&self, origin: &dyn BoardCase<T>, size: Vec<i64> )->dyn Board<T>;
        fn distance(&self, origin: &dyn BoardCase<T>, target: &dyn BoardCase<T>)->i64;
        fn get_dim(&self) -> Vec<i64>;
        fn print(&self);
    }

    pub mod square2d {
        pub struct Board2D< T : Eq + PartialEq + Sized > {
            board : Vec< Vec<T> >
        }
    }

    pub mod hexagon37 {
        pub struct Hexagon37Case< T : Eq + PartialEq + Sized > {
            line1 : i64,
            line2 : i64,
            line3 : i64,
            value: T,
        }

        pub struct Hexagon37< T : Eq + PartialEq + Sized > {
            board : Vec< Hexagon37Case<T> >
        }
    }




}
