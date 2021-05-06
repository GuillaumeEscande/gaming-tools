pub mod base {
    use std::vec::Vec;

    pub trait BoardCase< T: Eq + PartialEq + Sized > {
        fn position(&self)->Vec<i64>;
        fn get_value(&self)->T;
        fn distance(&self, target: &T)->i64;
        fn get_board(&self)->&dyn Board<T>;
    }

    pub trait Board< T : Eq + PartialEq + Sized > {
        fn neighbors(&self, origin: &dyn BoardCase<T> )->Vec<&dyn BoardCase<T> >;
        fn sub_board(&self, origin: &dyn BoardCase<T>, size: Vec<i64> )->dyn Board<T>;
        fn distance(&self, origin: &dyn BoardCase<T>, target: &dyn BoardCase<T>)->i64;
        fn get_dim(&self) -> Vec<i64>;
        fn print(&self);
    }

    pub struct Board2D< T : Eq + PartialEq + Sized > {
        board : Vec< Vec<T> >
    }

    impl< T : Eq + PartialEq + Sized > Board<T> for Board2D<T>{
        fn neighbors(&self, origin: &dyn BoardCase<T> )->Vec<&dyn BoardCase<T> >{

        }
        fn sub_board(&self, origin: &dyn BoardCase<T>, size: Vec<i64> )->dyn Board<T>{

        }
        fn distance(&self, origin: &dyn BoardCase<T>, target: &dyn BoardCase<T>)->i64{

        }
        fn get_dim(&self) -> Vec<i64>{

        }
        fn print(&self){
            
        }
    }




}