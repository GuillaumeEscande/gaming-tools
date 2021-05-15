use std::vec::Vec;
use std::fmt::Debug;
use std::rc::Rc;

pub trait State<T> : Debug {
    fn name(&self) -> &str;
    fn is_validated(&self, game: &T)->bool;
    fn next(&self)->Option<Rc<dyn State<T>>>;
}

pub struct Machine<T> {
    states: Vec<Rc<dyn State<T>>>,
    current_state: Rc<dyn State<T>>
}

impl<T> Machine <T> {
    fn compute(&mut self, game: &T) -> (){
        if self.current_state.is_validated(game){
            if self.current_state.next().is_some() {
                let next = self.current_state.next();
                self.current_state = next.unwrap();
            }
        }
    }
    fn states(&self) -> Vec<Rc<dyn State<T>>>{
        return self.states.clone();
    }
    fn state(&self) -> Rc<dyn State<T>>{
        return Rc::clone(&self.current_state);
    }
}
