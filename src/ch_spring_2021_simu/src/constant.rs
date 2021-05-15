

use std::vec::Vec;
use std::collections::HashMap;


pub struct Constant {
}

impl Constant {

    fn linear_mapping() -> Vec<Vec<i16>> {
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

        return id_maping;
    }

    pub fn linear_to_coord(linear: i16) -> Vec<i16> {
        let mapping = Constant::linear_mapping();
        return mapping[linear as usize].clone();
    }

    pub fn linear_vec_to_coord(linear: &Vec<i16>) -> Vec<i16> {
        let mapping = Constant::linear_mapping();
        return mapping[linear[0] as usize].clone();
    }

    fn coord_mapping() -> HashMap< Vec< i16 >, i16 > {
        let mut id_maping = HashMap::< Vec::< i16 >, i16 >::new();

        id_maping.insert(vec!( 0,  0,  0), 0);
        id_maping.insert(vec!( 1, -1,  0), 1);
        id_maping.insert(vec!( 1,  0, -1), 2);
        id_maping.insert(vec!( 0,  1, -1), 3);
        id_maping.insert(vec!(-1,  1,  0), 4);
        id_maping.insert(vec!(-1,  0,  1), 5);
        id_maping.insert(vec!( 0, -1,  1), 6);
        id_maping.insert(vec!( 2, -2,  0), 7);
        id_maping.insert(vec!( 2, -1, -1), 8);
        id_maping.insert(vec!( 2,  0, -2), 9);
        id_maping.insert(vec!( 1,  1, -2),10);
        id_maping.insert(vec!( 0,  2, -2),11);
        id_maping.insert(vec!(-1,  2, -1),12);
        id_maping.insert(vec!(-2,  2,  0),13);
        id_maping.insert(vec!(-2,  1,  1),14);
        id_maping.insert(vec!(-2,  0,  2),15);
        id_maping.insert(vec!(-1, -1,  2),16);
        id_maping.insert(vec!( 0, -2,  2),17);
        id_maping.insert(vec!( 1, -2,  1),18);
        id_maping.insert(vec!( 3, -3,  0),19);
        id_maping.insert(vec!( 3, -2, -1),20);
        id_maping.insert(vec!( 3, -1, -2),21);
        id_maping.insert(vec!( 3,  0, -3),22);
        id_maping.insert(vec!( 2,  1, -3),23);
        id_maping.insert(vec!( 1,  2, -3),24);
        id_maping.insert(vec!( 0,  3, -3),25);
        id_maping.insert(vec!(-1,  3, -2),26);
        id_maping.insert(vec!(-2,  3, -1),27);
        id_maping.insert(vec!(-3,  3,  0),28);
        id_maping.insert(vec!(-3,  2,  1),29);
        id_maping.insert(vec!(-3,  1,  2),30);
        id_maping.insert(vec!(-3,  0,  3),31);
        id_maping.insert(vec!(-2, -1,  3),32);
        id_maping.insert(vec!(-1, -2,  3),33);
        id_maping.insert(vec!( 0, -3,  3),34);
        id_maping.insert(vec!( 1, -3,  2),35);
        id_maping.insert(vec!( 2, -3,  1),36);

        return id_maping;
    }

    pub fn coor_to_linear(coord: &Vec<i16>) -> i16 {
        let mapping = Constant::coord_mapping();
        return *mapping.get(coord).unwrap();
    }

    pub fn case_assembly() -> Vec<Vec<i16>> {
        let prop1 = vec!( 0,  8, 10, 12, 14, 16, 18, 19, 22, 25, 28, 31, 34);
        let prop2 = vec!( 1,  3,  5,  9, 13, 17, 20, 24, 26, 30, 32, 36);
        let prop3 = vec!( 2,  4,  6,  7, 11, 15, 21, 23, 27, 29, 33, 35);
        return vec!(prop1, prop2, prop3);
    }
}
