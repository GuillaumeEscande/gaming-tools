use board::Board;
use board::BoardCase;
use std::vec::Vec;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;
use crate::constant;

#[derive(Debug)]
pub struct LinearHexagonCase< T : Eq + PartialEq + Sized + Debug > {
    id : i16,
    case : Rc<T>,
}

impl< T : Eq + PartialEq + Sized + Clone + Hash + Debug > BoardCase< T > for LinearHexagonCase< T > {
    fn position(&self)->Vec<i16>{
        return vec!(self.id);
    }
    fn get_value(&self)->&T{
        return &self.case.as_ref();
    }
    fn get_value_mut(&mut self)->&mut T{
        return Rc::get_mut(&mut self.case).unwrap();
    }
    fn set_value(&mut self, value : &T){
        self.case = Rc::new(value.clone());
    }
}

#[derive(Debug)]
pub struct LinearHexagon< T : Eq + PartialEq + Sized + Debug + Hash > {
    pub size: usize,
    values: Vec<Rc<dyn BoardCase<T>>>,
    neighbors: Vec<Vec<i16>>
}

impl< T : 'static + Eq + PartialEq + Sized + Clone + Debug + Hash > LinearHexagon<T>{
    pub fn new(size : usize, values: &Vec<Rc<T>>, neighbors : &Vec<Vec<i16>>) -> LinearHexagon<T>{

        let mut values_case : Vec<Rc<dyn BoardCase<T>>> = Vec::with_capacity(size);

        for i in 0..size{
            values_case.push(Rc::new(LinearHexagonCase::<T>{
                id: i as i16,
                case: Rc::clone(&values[i])
            }));
        }

        let result = LinearHexagon::<T>{
            size: size,
            values: values_case,
            neighbors: neighbors.clone()
        };
        return result;
    }
}

impl< T : Eq + PartialEq + Sized + Clone + Hash + Debug > Board< T > for LinearHexagon< T > {
    fn neighbors(&self, origin: &Rc<dyn BoardCase<T>> )->Vec<Rc<dyn BoardCase<T>> >{
        let neighbors_i : &Vec::<i16> = &self.neighbors[origin.position()[0] as usize];
        let mut extract : Vec<Rc<dyn BoardCase<T>> > = Vec::new();
        for i in neighbors_i {
            extract.push(Rc::clone(&self.values[*i as usize]));
        }
        return extract;
    }
    fn distance(&self, origin: &Rc<dyn BoardCase<T>>, target: &Rc<dyn BoardCase<T>>)->i16{
        use std::cmp;
        let origin_pos = constant::Constant::linear_vec_to_coord(&origin.position());
        let target_pos = constant::Constant::linear_vec_to_coord(&target.position());
        let mut distance : i16 = 0;
        
        for c in 0..3 {
            distance = cmp::max(distance, (origin_pos[c] as i16 - target_pos[c] as i16).abs());
        }
        return distance;
    }
    fn size(&self) -> Vec<usize>{
        return vec!(self.size);
    }

    fn get(&self, pos: &Vec<i16>)->Rc<dyn BoardCase<T>>{
        return Rc::clone(&self.values[pos[0] as usize]);
    }

    fn print(&self){
    }
}
