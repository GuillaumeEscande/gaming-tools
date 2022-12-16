use std::vec::Vec;
use std::fmt::Debug;
use std::rc::Rc;
use board::Board;
use board::BoardCase;
use std::marker::PhantomData;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct HexagonCase< T : Eq + PartialEq + Sized + Debug + Clone > {
    line1 : i16,
    line2 : i16,
    line3 : i16,
    value: Rc<T>,
}

impl< T : Eq + PartialEq + Sized + Debug + Clone > BoardCase< T > for HexagonCase< T > {
    fn position(&self)->Vec<i16>{
        return vec!(self.line1, self.line2, self.line3);
    }
    fn get_value(&self)->&Rc<T>{
        return &self.value;
    }
    fn set_value(&mut self, value : &Rc<T>){
        self.value = Rc::clone(value);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Hexagon< T : Eq + PartialEq + Sized + Debug + Clone > {
    size: usize,
    board : Vec< Vec< Vec< Rc< HexagonCase< T > > > > >,
    linear : Vec<Rc< HexagonCase< T > > >,
    _phantom_t: PhantomData<T>,
}

impl< T : Eq + PartialEq + Sized + Debug + Clone > Hexagon< T >{
    pub fn new(size : usize, default: &T) -> Hexagon<T>{

        let mut board = Vec::with_capacity(size);
        let mut linear = Vec::new();
        let real_size = 1 + size * 2;

        for c1 in 0..real_size{
            board.push(Vec::with_capacity(real_size));
            for c2 in 0..real_size{
                board[c1].push(Vec::with_capacity(real_size));
                for c3 in 0..real_size{
                    let case : Rc< HexagonCase< T > > = Rc::new(HexagonCase::<T>{
                        line1 : c1 as i16 - size as i16,
                        line2 : c2 as i16 - size as i16,
                        line3 : c3 as i16 - size as i16,
                        value: Rc::new(default.clone()),
                    });
                    linear.push(Rc::clone(&case));
                    board[c1][c2].push(case);
                }
            }
        }
        let result = Hexagon::<T>{
            board: board,
            linear: linear,
            size: size,
            _phantom_t: PhantomData
        };
        return result;
    }
}

impl< T : Eq + PartialEq + Sized + Debug + Clone> Board< T, HexagonCase< T > > for Hexagon< T > {
    fn neighbors(&self, origin: &Rc<HexagonCase< T >> )->Vec<Rc<HexagonCase< T >> >{
        let pos1 : usize = (origin.position()[0] + self.size as i16) as usize;
        let pos2 : usize = (origin.position()[1] + self.size as i16) as usize;
        let pos3 : usize = (origin.position()[2] + self.size as i16) as usize;

        let mut neighbors : Vec<Rc<HexagonCase< T >>> = Vec::with_capacity(6);

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
    fn distance(&self, origin: &Rc<HexagonCase< T >>, target: &Rc<HexagonCase< T >>)->i16{
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

    fn get(&self, pos: &Vec<i16>)->&Rc<HexagonCase< T >>{
        let pos1 : usize = (pos[0] + self.size as i16) as usize;
        let pos2 : usize = (pos[1] + self.size as i16) as usize;
        let pos3 : usize = (pos[2] + self.size as i16) as usize;

        return &self.board[pos1][pos2][pos3];
    }
    fn print(&self){
    }
    fn to_linear(&self)->&Vec<Rc<HexagonCase<T>>>{
        return &self.linear;
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




