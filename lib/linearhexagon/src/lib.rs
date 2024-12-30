use board::Board;
use board::BoardCase;
use std::vec::Vec;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct LinearHexagonCase< T : Eq + PartialEq + Sized + Debug + Clone > {
    id : usize,
    value : Rc<T>,
}

impl< T : Eq + PartialEq + Sized + Debug + Clone > BoardCase< T > for LinearHexagonCase< T > {
    fn position(&self)->Vec<i16>{
        return vec!(self.id as i16);
    }
    fn get_value(&self)->&Rc<T>{
        return &self.value;
    }
    fn set_value(&mut self, value : &Rc<T>){
        self.value = Rc::clone(value);
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct LinearHexagon< T : Eq + PartialEq + Sized + Debug + Clone > {
    pub size: usize,
    values: Vec<Rc<LinearHexagonCase<T>>>,
    neighbors: Vec<Vec<i16>>,
    coord_mapping: Vec<Vec<i16>>
}

impl< T : Eq + PartialEq + Sized + Debug + Clone > LinearHexagon<T>{
    pub fn new(size : usize, values: &Vec<Rc<T>>, neighbors : &Vec<Vec<i16>>) -> LinearHexagon<T>{

        let mut values_case : Vec<Rc<LinearHexagonCase<T>>> = Vec::with_capacity(size);

        for i in 0..size{
            values_case.push(Rc::new(LinearHexagonCase::<T>{
                id: i ,
                value: Rc::clone(&values[i])
            }));
        }

        let mut id_maping = Vec::with_capacity(37);

        id_maping.push(vec!( 0,  0,  0));
        id_maping.push(vec!( 1, -1,  0));
        id_maping.push(vec!( 1,  0, -1));
        id_maping.push(vec!( 0,  1, -1));
        id_maping.push(vec!(-1,  1,  0));
        id_maping.push(vec!(-1,  0,  1));
        id_maping.push(vec!( 0, -1,  1));
        id_maping.push(vec!( 2, -2,  0));
        id_maping.push(vec!( 2, -1, -1));
        id_maping.push(vec!( 2,  0, -2));
        id_maping.push(vec!( 1,  1, -2));
        id_maping.push(vec!( 0,  2, -2));
        id_maping.push(vec!(-1,  2, -1));
        id_maping.push(vec!(-2,  2,  0));
        id_maping.push(vec!(-2,  1,  1));
        id_maping.push(vec!(-2,  0,  2));
        id_maping.push(vec!(-1, -1,  2));
        id_maping.push(vec!( 0, -2,  2));
        id_maping.push(vec!( 1, -2,  1));
        id_maping.push(vec!( 3, -3,  0));
        id_maping.push(vec!( 3, -2, -1));
        id_maping.push(vec!( 3, -1, -2));
        id_maping.push(vec!( 3,  0, -3));
        id_maping.push(vec!( 2,  1, -3));
        id_maping.push(vec!( 1,  2, -3));
        id_maping.push(vec!( 0,  3, -3));
        id_maping.push(vec!(-1,  3, -2));
        id_maping.push(vec!(-2,  3, -1));
        id_maping.push(vec!(-3,  3,  0));
        id_maping.push(vec!(-3,  2,  1));
        id_maping.push(vec!(-3,  1,  2));
        id_maping.push(vec!(-3,  0,  3));
        id_maping.push(vec!(-2, -1,  3));
        id_maping.push(vec!(-1, -2,  3));
        id_maping.push(vec!( 0, -3,  3));
        id_maping.push(vec!( 1, -3,  2));
        id_maping.push(vec!( 2, -3,  1));

        let result = LinearHexagon::<T>{
            size: size,
            values: values_case,
            neighbors: neighbors.clone(),
            coord_mapping: id_maping
        };
        return result;
    }
}

impl< T : Eq + PartialEq + Sized + Debug + Clone > Board< T, LinearHexagonCase<T> > for LinearHexagon< T > {
    fn neighbors(&self, origin: &Rc<LinearHexagonCase<T>> )->Vec<Rc<LinearHexagonCase<T>> >{
        let neighbors_i : &Vec::<i16> = &self.neighbors[origin.position()[0] as usize];
        let mut extract : Vec<Rc<LinearHexagonCase<T>> > = Vec::new();
        for i in neighbors_i {
            extract.push(Rc::clone(&self.values[*i as usize]));
        }
        return extract;
    }
    fn distance(&self, origin: &Rc<LinearHexagonCase<T>>, target: &Rc<LinearHexagonCase<T>>)->i16{
        use std::cmp;
        let origin_pos = &self.coord_mapping[origin.position()[0] as usize];
        let target_pos = &self.coord_mapping[target.position()[0] as usize];
        let mut distance : i16 = 0;

        for c in 0..3 {
            distance = cmp::max(distance, (origin_pos[c] as i16 - target_pos[c] as i16).abs());
        }
        return distance;
    }
    fn size(&self) -> Vec<usize>{
        return vec!(self.size);
    }

    fn get(&self, pos: &Vec<i16>)->&Rc<LinearHexagonCase<T>>{
        return &self.values[pos[0] as usize];
    }

    fn print(&self){
    }
    fn to_linear(&self)->&Vec<Rc<LinearHexagonCase<T>>>{
        return &self.values;
    }
}
