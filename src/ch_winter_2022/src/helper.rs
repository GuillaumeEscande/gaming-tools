use crate::model;
use std::rc::Rc;
use board::BoardCase;
use board::Board;




pub fn compute_recycle_porentiel(case : &Rc<board2d::Case2D<model::Case>>, board: &board2d::Board2D<model::Case>) -> i16{
    let mut scrap = case.get_value().scrap_amount;
    board.neighbors(case).iter().for_each(|case| {scrap += case.get_value().scrap_amount});
    return scrap;
}