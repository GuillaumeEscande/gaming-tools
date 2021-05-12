use crate::linear_hexagon;
use crate::model;
use board::Board;

pub fn sort_better_cases(board: &linear_hexagon::LinearHexagon::<model::Case>) -> Vec<i16>{
    vec!(0, 1, 2, 3, 4, 5, 6)
}