use std::vec::Vec;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;

pub trait BoardCase< T: Eq + PartialEq + Sized + Hash + Debug > : Debug {
    fn position(&self)->Vec<i16>;
    fn get_value(&self)->&T;
    fn get_value_mut(&mut self)->&mut T;
    fn set_value(&mut self, value : &T);
}

pub trait Board< T : Eq + PartialEq + Sized + Debug > : Debug {
    fn neighbors(&self, origin: &Rc<dyn BoardCase<T>> )->Vec<Rc<dyn BoardCase<T>>>;
    fn distance(&self, origin: &Rc<dyn BoardCase<T>>, target: &Rc<dyn BoardCase<T>>)->i16;
    fn get(&self, pos: &Vec<i16>)->Rc<dyn BoardCase<T>>;
    fn size(&self) -> Vec<usize>;
    fn print(&self);
}


#[derive(Debug)]
pub struct HexagonCase< T : Eq + PartialEq + Sized + Debug > {
    line1 : i16,
    line2 : i16,
    line3 : i16,
    value: T,
}

impl< T : Eq + PartialEq + Sized + Clone + Hash + Debug > BoardCase< T > for HexagonCase< T > {
    fn position(&self)->Vec<i16>{
        return vec!(self.line1, self.line2, self.line3);
    }
    fn get_value(&self)->&T{
        return &self.value;
    }
    fn get_value_mut(&mut self)->&mut T{
        return &mut self.value;
    }
    fn set_value(&mut self, value : &T){
        self.value = value.clone();
    }
}

#[derive(Debug)]
pub struct Hexagon< T : Eq + PartialEq + Sized + Debug + Hash > {
    size: usize,
    board : Vec< Vec< Vec< Rc< dyn BoardCase< T > > > > >
}

impl< T : 'static + Eq + PartialEq + Sized + Clone + Debug + Hash > Hexagon<T>{
    pub fn new(size : usize, default: &T) -> Hexagon<T>{

        let mut board = Vec::with_capacity(size);
        let real_size = 1 + size * 2;

        for c1 in 0..real_size{
            board.push(Vec::with_capacity(real_size));
            for c2 in 0..real_size{
                board[c1].push(Vec::with_capacity(real_size));
                for c3 in 0..real_size{
                    let case : Rc< dyn BoardCase< T > > = Rc::new(HexagonCase::<T>{
                        line1 : c1 as i16 - size as i16,
                        line2 : c2 as i16 - size as i16,
                        line3 : c3 as i16 - size as i16,
                        value: default.clone(),
                    });
                    board[c1][c2].push(case);
                }
            }
        }
        let result = Hexagon::<T>{
            board: board,
            size: size
        };
        return result;
    }
}

impl< T : Eq + PartialEq + Sized + Clone + Hash + Debug > Board< T > for Hexagon< T > {
    fn neighbors(&self, origin: &Rc<dyn BoardCase<T>> )->Vec<Rc<dyn BoardCase<T>> >{
        let pos1 : usize = (origin.position()[0] + self.size as i16) as usize;
        let pos2 : usize = (origin.position()[1] + self.size as i16) as usize;
        let pos3 : usize = (origin.position()[2] + self.size as i16) as usize;

        let mut neighbors : Vec<Rc<dyn BoardCase<T>>> = Vec::with_capacity(6);

        let real_size : usize = 1 + self.size * 2;

        if pos1 > 1 && (pos2 < (real_size - 1)) {
            neighbors.push(Rc::clone(&self.board[pos1-1][pos2+1][pos3  ]));
        }
        if pos2 > 1 && (pos3 < (real_size - 1)) {
            neighbors.push(Rc::clone(&self.board[pos1  ][pos2-1][pos3+1]));
        }
        if pos3 > 1 && (pos1 < (real_size - 1)) {
            neighbors.push(Rc::clone(&self.board[pos1+1][pos2  ][pos3-1]));
        }
        if pos2 > 1 && (pos1 < (real_size - 1)) {
            neighbors.push(Rc::clone(&self.board[pos1+1][pos2-1][pos3  ]));
        }
        if pos3 > 1 && (pos2 < (real_size - 1)) {
            neighbors.push(Rc::clone(&self.board[pos1  ][pos2+1][pos3-1]));
        }
        if pos1 > 1 && (pos3 < (real_size - 1)) {
            neighbors.push(Rc::clone(&self.board[pos1-1][pos2  ][pos3+1]));
        }

        return neighbors;
    }
    fn distance(&self, origin: &Rc<dyn BoardCase<T>>, target: &Rc<dyn BoardCase<T>>)->i16{
        use std::cmp;
        let mut distance : i16 = 0;
        
        for c in 0..3 {
            distance = cmp::max(distance, (origin.position()[c] as i16 - target.position()[c] as i16).abs());
        }
        return distance;
    }
    fn size(&self) -> Vec<usize>{
        return vec!(self.size, self.size, self.size);
    }

    fn get(&self, pos: &Vec<i16>)->Rc<dyn BoardCase<T>>{
        let pos1 : usize = (pos[0] + self.size as i16) as usize;
        let pos2 : usize = (pos[1] + self.size as i16) as usize;
        let pos3 : usize = (pos[2] + self.size as i16) as usize;

        return Rc::clone(&self.board[pos1][pos2][pos3]);
    }

    fn print(&self){
        /*
        let real_size = (1 + self.size * 2) as i16;

        let isize : i16 = self.size as i16;

        println!("Hexagon :");
        for q in 0..real_size{
            print!("|");
            for r in 0..real_size{
                if r > q && r < real_size - 1 - q {
                    let x = q;
                    let z = r;
                    let y = (-1 * x) - z;

                    //print!("{:?} {} {} {}",self.size, x, y, z );
                    let xu = (x + self.size as i16) as usize;
                    let yu = (y + self.size as i16) as usize;
                    let zu = (z + self.size as i16) as usize;

                    //let value = &self.board[xu][yu][zu];
                    print!("O");
                } else {
                    print!(" ")
                }
            }
            print!("|");
            println!("");
        }
        */
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal() {
        let board = Hexagon::<i16>::new( 3, &0);
        assert_eq!(board.size(), vec!(3, 3, 3));
        assert_eq!(board.neighbors( &board.get( &vec!(3, -3, 0) ) ).len(), 3);
        assert_eq!(board.neighbors( &board.get( &vec!(0, -3, 3) ) ).len(), 3);
        assert_eq!(board.neighbors( &board.get( &vec!(-3, 2, 1) ) ).len(), 4);
        assert_eq!(board.neighbors( &board.get( &vec!(-1, -1, 2) ) ).len(), 6);
        assert_eq!(board.neighbors( &board.get( &vec!(0, 0, 0) ) ).len(), 6);

        assert_eq!(board.distance( &board.get( &vec!(0, 0, 0) ), &board.get( &vec!(0, 0, 0) ) ), 0);
        assert_eq!(board.distance( &board.get( &vec!(0, 0, 0) ), &board.get( &vec!(-1, 1, 0) ) ), 1);
        assert_eq!(board.distance( &board.get( &vec!(0, 0, 0) ), &board.get( &vec!(1, 1, -2) ) ), 2);
        assert_eq!(board.distance( &board.get( &vec!(0, 0, 0) ), &board.get( &vec!(3, -2, -1) ) ), 3);
        assert_eq!(board.distance( &board.get( &vec!(-3, 0, 3) ), &board.get( &vec!(2, -3, 1) ) ), 5);
        assert_eq!(board.distance( &board.get( &vec!(1, -3, 2) ), &board.get( &vec!(0, 3, -3) ) ), 6);

        board.print();
    }
}   




