use std::vec::Vec;
use std::fmt::Debug;
use std::rc::Rc;

pub trait BoardCase< T: Eq + PartialEq + Sized + Debug > : Debug + Eq + PartialEq + Clone + Sized  {
    fn position(&self)->Vec<i16>;
    fn get_value(&self)->&Rc<T>;
    fn get_mut_value(&mut self)->&mut Rc<T>;
    fn set_value(&mut self, value : &Rc<T>);
}

pub trait Board< T : Eq + PartialEq + Sized + Debug ,  C : BoardCase<T> > : Debug + Eq + PartialEq + Clone + Sized {
    fn neighbors(&self, origin: &Rc<C> )->Vec<Rc<C>>;
    fn distance(&self, origin: &Rc<C>, target: &Rc<C>)->i16;
    fn get(&self, pos: &Vec<i16>)->&Rc<C>;
    fn get_mut(&mut self, pos: &Vec<i16>)->&mut Rc<C>;
    fn size(&self) -> Vec<usize>;
    fn print(&self);
}


