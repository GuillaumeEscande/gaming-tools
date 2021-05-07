pub mod base {
    use std::vec::Vec;

    pub trait BoardCase< T: Eq + PartialEq + Sized > {
        fn position(&self)->Vec<i64>;
        fn get_value(&self)->&T;
    }

    pub trait Board< T : Eq + PartialEq + Sized > {
        fn neighbors(&self, origin: &dyn BoardCase<T> )->Vec<&dyn BoardCase<T> >;
        fn distance(&self, origin: &dyn BoardCase<T>, target: &dyn BoardCase<T>)->i64;
        fn get(&self, pos: &Vec<i64>)->&dyn BoardCase<T>;
        fn size(&self) -> Vec<usize>;
        fn print(&self);
    }

    pub mod hexagon {
        use crate::board::base;

        pub struct HexagonCase< T : Eq + PartialEq + Sized > {
            line1 : i64,
            line2 : i64,
            line3 : i64,
            value: T,
        }

        impl< T : Eq + PartialEq + Sized > base::BoardCase< T > for HexagonCase< T > {
            fn position(&self)->Vec<i64>{
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
                let real_size = 1 + size * 2;

                for c1 in 0..real_size{
                    board.push(Vec::with_capacity(real_size));
                    for c2 in 0..real_size{
                        board[c1].push(Vec::with_capacity(real_size));
                        for c3 in 0..real_size{
                            let case = HexagonCase::<T>{
                                line1 : c1 as i64 - size as i64,
                                line2 : c2 as i64 - size as i64,
                                line3 : c3 as i64 - size as i64,
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
                let pos1 : usize = (origin.position()[0] + self.size as i64) as usize;
                let pos2 : usize = (origin.position()[1] + self.size as i64) as usize;
                let pos3 : usize = (origin.position()[2] + self.size as i64) as usize;

                let real_size = 1 + self.size * 2;

                let mut neighbors : Vec<&dyn base::BoardCase<T>> = Vec::with_capacity(6);

                if pos1 > 1 {
                    neighbors.push(&self.board[pos1-1][pos2][pos3]);
                }
                if pos1 < real_size - 2 {
                    neighbors.push(&self.board[pos1+1][pos2][pos3]);
                }
                if pos2 > 1 {
                    neighbors.push(&self.board[pos1][pos2-1][pos3]);
                }
                if pos2 < real_size -2 {
                    neighbors.push(&self.board[pos1][pos2+1][pos3]);
                }
                if pos3 > 1 {
                    neighbors.push(&self.board[pos1][pos2][pos3-1]);
                }
                if pos3 < real_size -2 {
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

            fn get(&self, pos: &Vec<i64>)->&dyn base::BoardCase<T>{
                let pos1 : usize = (pos[0] + self.size as i64) as usize;
                let pos2 : usize = (pos[1] + self.size as i64) as usize;
                let pos3 : usize = (pos[2] + self.size as i64) as usize;

                &self.board[pos1][pos2][pos3]
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
        assert_eq!(board.neighbors( board.get( &vec!(3, 3, 0) ) ).len(), 3);
        assert_eq!(board.neighbors( board.get( &vec!(0, -3, 3) ) ).len(), 3);
        assert_eq!(board.neighbors( board.get( &vec!(-3, 2, 1) ) ).len(), 3);
        assert_eq!(board.neighbors( board.get( &vec!(-1, -1, 2) ) ).len(), 6);
        assert_eq!(board.neighbors( board.get( &vec!(0, 0, 0) ) ).len(), 6);

        assert_eq!(board.distance( board.get( &vec!(0, 0, 0) ), board.get( &vec!(0, 0, 0) ) ), 0);
        assert_eq!(board.distance( board.get( &vec!(0, 0, 0) ), board.get( &vec!(1, 0, 0) ) ), 1);
        assert_eq!(board.distance( board.get( &vec!(0, 0, 0) ), board.get( &vec!(0, 3, 0) ) ), 3);
        assert_eq!(board.distance( board.get( &vec!(0, 0, 0) ), board.get( &vec!(1, 2, 1) ) ), 2);

    }
}   