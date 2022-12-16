use std::io;
use crate::model;
use crate::game_impl;
use std::rc::Rc;

#[warn(unused_macros)]
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


pub fn init_size() -> (usize, usize) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32);
    let height = parse_input!(inputs[1], i32);

    return (width as usize, height as usize);

}

pub fn init_game(size : (usize, usize), previous_turn: i16) -> game_impl::GameImpl {

    let width = size.0;
    let height = size.1;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();

    let my_matter = parse_input!(inputs[0], i16);
    let me = model::Player{id: 1, matter: my_matter};
    let opp_matter = parse_input!(inputs[1], i16);
    let opp = model::Player{id: 0, matter: opp_matter};

    let mut cases : Vec<Rc<model::Case>> = Vec::with_capacity(height * width);

    for _ in 0..height as usize {
        for _ in 0..width as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let scrap_amount = parse_input!(inputs[0], i16);
            let owner = parse_input!(inputs[1], i16); // 1 = me, 0 = foe, -1 = neutral
            let units = parse_input!(inputs[2], i16);
            let recycler = parse_input!(inputs[3], i16);
            let can_build = parse_input!(inputs[4], i32);
            let can_spawn = parse_input!(inputs[5], i32);
            let in_range_of_recycler = parse_input!(inputs[6], i16);

            let mut case_type = model::CaseType::GRASS();
            if owner >= 0 {
                case_type = model::CaseType::EMPTY();
            }
            if recycler > 0 {
                case_type = model::CaseType::RECYCLER();
            }
            if units > 0 {
                case_type = model::CaseType::ROBOT(units);
            }


            cases.push(Rc::new(model::Case{
                scrap_amount: scrap_amount,
                owner_id: owner,
                case_type: case_type,
                units: units,
                can_build: can_build != 0,
                can_spawn: can_spawn != 0,
                in_range_of_recycler: in_range_of_recycler != 0,
            }));
        }
    }

    
    let board : board2d::Board2D::<model::Case> = board2d::Board2D::<model::Case>::new(
        width as usize,
        height as usize,
        &cases,
    );


    let game : game_impl::GameImpl = game_impl::GameImpl{
        board: board,
        turn: previous_turn + 1,
        me: me,
        opp: opp
    };

    return game;


}