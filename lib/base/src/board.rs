pub mod base {
    use std::vec::Vec;

    pub trait BoardCase< T: Eq + PartialEq + Sized > {
        fn position(&self)->Vec<usize>;
        fn get_value(&self)->&T;
    }

    pub trait Board< T : Eq + PartialEq + Sized > {
        fn neighbors(&self, origin: &dyn BoardCase<T> )->Vec<&dyn BoardCase<T> >;
        //fn sub_board(&self, origin: &dyn BoardCase<T>, size: Vec<i64> )->dyn Board<T>;
        fn distance(&self, origin: &dyn BoardCase<T>, target: &dyn BoardCase<T>)->i64;
        fn get(&self, pos: &Vec<usize>)->&dyn BoardCase<T>;
        fn size(&self) -> Vec<usize>;
        fn print(&self);
    }

    pub mod hexagon {
        use crate::board::base;

        pub struct HexagonCase< T : Eq + PartialEq + Sized > {
            line1 : usize,
            line2 : usize,
            line3 : usize,
            value: T,
        }

        impl< T : Eq + PartialEq + Sized > base::BoardCase< T > for HexagonCase< T > {
            fn position(&self)->Vec<usize>{
                return vec!(self.line1, self.line2, self.line3);
            }
            fn get_value(&self)->&T{
                return &self.value;
            }
        }

        pub struct Hexagon< T : Eq + PartialEq + Sized > {
            size: usize,
            board : Vec< Vec< Vec< HexagonCase<T> > > >
        }

        impl< T : Eq + PartialEq + Sized + Clone > Hexagon<T>{
            pub fn new(size : usize, default: &T) -> Hexagon<T>{

                let mut board = Vec::with_capacity(size);

                for c1 in 0..size{
                    board.push(Vec::with_capacity(size));
                    for c2 in 0..size{
                        board[c1].push(Vec::with_capacity(size));
                        for c3 in 0..size{
                            let case = HexagonCase::<T>{
                                line1 : c1,
                                line2 : c2,
                                line3 : c3,
                                value: default.clone(),
                            };
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

        impl< T : Eq + PartialEq + Sized > base::Board< T > for Hexagon< T > {
            fn neighbors(&self, origin: &dyn base::BoardCase<T> )->Vec<&dyn base::BoardCase<T> >{
                let pos1 : usize = origin.position()[0] as usize;
                let pos2 : usize = origin.position()[1] as usize;
                let pos3 : usize = origin.position()[2] as usize;

                let mut neighbors : Vec<&dyn base::BoardCase<T>> = Vec::with_capacity(6);

                if pos1 > 1 {
                    neighbors.push(&self.board[pos1-1][pos2][pos3]);
                }
                if pos1 < self.size - 2 {
                    neighbors.push(&self.board[pos1+1][pos2][pos3]);
                }
                if pos2 > 1 {
                    neighbors.push(&self.board[pos1][pos2-1][pos3]);
                }
                if pos2 < self.size -2 {
                    neighbors.push(&self.board[pos1][pos2+1][pos3]);
                }
                if pos3 > 1 {
                    neighbors.push(&self.board[pos1][pos2][pos3-1]);
                }
                if pos3 < self.size -2 {
                    neighbors.push(&self.board[pos1][pos2][pos3+1]);
                }
                return neighbors;
            }
            fn distance(&self, origin: &dyn base::BoardCase<T>, target: &dyn base::BoardCase<T>)->i64{
                let mut distance : i64 = 0;

                for c in 0..3 {
                    distance += (origin.position()[c] as i64 - target.position()[c] as i64).pow(2);
                }
                return (distance as f64).sqrt() as i64;
            }
            fn size(&self) -> Vec<usize>{
                return vec!(self.size, self.size, self.size);
            }

            fn get(&self, pos: &Vec<usize>)->&dyn base::BoardCase<T>{
                &self.board[pos[0]][pos[1]][pos[2]]
            }
            fn print(&self){
                //TODO
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::board::base::*;
    use crate::board::base::hexagon::*;

    #[test]
    fn test_nominal() {
        let board = Hexagon::<i64>::new( 7, &0);
        assert_eq!(board.size(), vec!(7, 7, 7));
        assert_eq!(board.neighbors( board.get( &vec!(3 as usize, 3 as usize, 3 as usize) ) ).len(), 6);
        assert_eq!(board.neighbors( board.get( &vec!(0 as usize, 0 as usize, 0 as usize) ) ).len(), 3);

        assert_eq!(board.distance( board.get( &vec!(0 as usize, 0 as usize, 0 as usize) ), board.get( &vec!(0 as usize, 0 as usize, 0 as usize) ) ), 0);
        assert_eq!(board.distance( board.get( &vec!(0 as usize, 0 as usize, 0 as usize) ), board.get( &vec!(1 as usize, 0 as usize, 0 as usize) ) ), 1);
        assert_eq!(board.distance( board.get( &vec!(0 as usize, 0 as usize, 0 as usize) ), board.get( &vec!(0 as usize, 3 as usize, 0 as usize) ) ), 3);
        assert_eq!(board.distance( board.get( &vec!(0 as usize, 0 as usize, 0 as usize) ), board.get( &vec!(1 as usize, 2 as usize, 1 as usize) ) ), 2);

    }
}   