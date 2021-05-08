
#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub enum Action {
    WAIT(String),
    SEED(i8, i8),
    GROW(i8),
    COMPLETE(i8),
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Case {
    pub tree: i8,
    pub richness: i8,
    pub is_mine: bool,
    pub is_dormant: bool
}

use std::collections::HashMap;

pub fn getIdMapping() -> HashMap<i16, Vec<i16>> {
    let mut id_maping = HashMap::with_capacity(37);

    id_maping.insert( 0, vec!( 0,  0,  0));
    id_maping.insert( 1, vec!( 1, -1,  0));
    id_maping.insert( 2, vec!( 1,  0, -1));
    id_maping.insert( 3, vec!( 0,  1, -1));
    id_maping.insert( 4, vec!(-1,  1,  0));
    id_maping.insert( 5, vec!(-1,  0,  1));
    id_maping.insert( 6, vec!( 0, -1,  1));
    id_maping.insert( 7, vec!( 2, -2,  0));
    id_maping.insert( 8, vec!( 2, -1, -1));
    id_maping.insert( 9, vec!( 2,  0, -2));
    id_maping.insert(10, vec!( 1,  1, -2));
    id_maping.insert(11, vec!( 0,  2, -2));
    id_maping.insert(12, vec!(-1,  2, -1));
    id_maping.insert(13, vec!(-2,  2,  0));
    id_maping.insert(14, vec!(-2,  1,  1));
    id_maping.insert(15, vec!(-2,  0,  2));
    id_maping.insert(16, vec!(-1, -1,  2));
    id_maping.insert(17, vec!( 0, -2,  2));
    id_maping.insert(18, vec!( 1, -2,  1));
    id_maping.insert(19, vec!( 3, -3,  0));
    id_maping.insert(20, vec!( 3, -2, -1));
    id_maping.insert(21, vec!( 3, -1, -2));
    id_maping.insert(22, vec!( 3,  0, -3));
    id_maping.insert(23, vec!( 2,  1, -3));
    id_maping.insert(24, vec!( 1,  2, -3));
    id_maping.insert(25, vec!( 0,  3, -3));
    id_maping.insert(26, vec!(-1,  3, -2));
    id_maping.insert(27, vec!(-2,  3, -1));
    id_maping.insert(28, vec!(-3,  3,  0));
    id_maping.insert(29, vec!(-3,  2,  1));
    id_maping.insert(30, vec!(-3,  1,  2));
    id_maping.insert(31, vec!(-3,  0,  3));
    id_maping.insert(32, vec!(-2, -1,  3));
    id_maping.insert(33, vec!(-1, -2,  3));
    id_maping.insert(34, vec!( 0, -3,  3));
    id_maping.insert(35, vec!( 1, -3,  2));
    id_maping.insert(36, vec!( 2, -3,  1));

    return id_maping;
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Player {
    pub sun: i32,
    pub score: i32,
    pub is_asleep: bool
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Game {
    pub day: i32,
    pub nextCompleteScore: i32,
    pub myPlayer: Player,
    pub vsPlayer: Player,
    pub actions: Vec<Action>
}