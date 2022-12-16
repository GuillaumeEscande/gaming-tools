use board::Board;
use board::BoardCase;
use std::vec::Vec;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Case2D< T : Eq + PartialEq + Sized + Debug + Clone > {
    x: usize,
    y: usize,
    n: usize,
    value : Rc<T>,
}

impl< T : Eq + PartialEq + Sized + Debug + Clone > BoardCase< T > for Case2D< T > {
    fn position(&self)->Vec<i16>{
        return vec!(self.x as i16, self.y as i16);
    }
    fn get_value(&self)->&Rc<T>{
        return &self.value;
    }
    fn set_value(&mut self, value : &Rc<T>){
        self.value = Rc::clone(value);
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Board2D< T : Eq + PartialEq + Sized + Debug + Clone > {
    pub width: usize,
    pub height: usize,
    cases: Vec<Rc<Case2D<T>>>,
}

impl< T : Eq + PartialEq + Sized + Debug + Clone > Board2D<T>{
    pub fn new(width: usize, height: usize, values: &Vec<Rc<T>>) -> Board2D<T>{

        let mut cases : Vec<Rc<Case2D<T>>> = Vec::with_capacity(height * width);

        for h in 0..height{
            for w in 0..width{
                let n = (h*width + w) as usize;
                cases.push(Rc::new(Case2D::<T>{
                    x: w,
                    y: h,
                    n: n,
                    value: Rc::clone(&values[n])
                }));
            }
        }

        let result = Board2D::<T>{
            width: width,
            height: height,
            cases: cases
        };
        return result;
    }
}

impl< T : Eq + PartialEq + Sized + Debug + Clone > Board< T, Case2D<T> > for Board2D< T > {
    fn neighbors(&self, origin: &Rc<Case2D<T>> )->Vec<Rc<Case2D<T>> >{
        let mut neighbors : Vec<Rc<Case2D<T>> > = Vec::new();
        let x = origin.position()[0] as usize;
        let y = origin.position()[1] as usize;
        // Left
        if x > 0 {
            neighbors.push(Rc::clone(&self.cases[(x-1) + y*self.width]));
        }
        // Right
        if x < self.width - 1 {
            neighbors.push(Rc::clone(&self.cases[(x+1) + y*self.width]));
        }
        // Top
        if y > 0 {
            neighbors.push(Rc::clone(&self.cases[x+(y-1)*self.width]));
        }
        // Bottom
        if y < self.height - 1 {
            neighbors.push(Rc::clone(&self.cases[x+(y+1)*self.width]));
        }


        return neighbors;
    }
    fn distance(&self, origin: &Rc<Case2D<T>>, target: &Rc<Case2D<T>>)->i16{
        let x_diff = target.position()[0] - origin.position()[0];
        let y_diff = target.position()[1] - origin.position()[1];

        return f32::sqrt((x_diff.pow(2) + y_diff.pow(1)) as f32).round() as i16;
    }
    fn size(&self) -> Vec<usize>{
        return vec!(self.width, self.height);
    }

    fn get(&self, pos: &Vec<i16>)->&Rc<Case2D<T>>{
        return &self.cases[(pos[0] + pos[1] * (self.width as i16) ) as usize];
    }

    fn print(&self){
    }
    fn to_linear(&self)->&Vec<Rc<Case2D<T>>>{
        return &self.cases;
    }
}
