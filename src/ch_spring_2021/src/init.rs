use std::io;
use std::collections::HashMap;
use board::Hexagon;
use crate::model;
use board::Board;


#[warn(unused_macros)]
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


pub fn init_board( id_mapping: &HashMap<i16, Vec<i16>> ) -> Hexagon::<model::Case> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_cells = parse_input!(input_line, i32); // 37
    let mut board : Hexagon::<model::Case> = Hexagon::<model::Case>::new(3, &model::Case{
        tree: -1,
        richness: -1,
        is_mine: false,
        is_dormant: false
    });
    for i in 0..number_of_cells as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let index = parse_input!(inputs[0], i16); // 0 is the center cell, the next cells spiral outwards
        let richness = parse_input!(inputs[1], i8); // 0 if the cell is unusable, 1-3 for usable cells
        board.get_mut( &id_mapping[&index] ).get_value_mut().richness = richness;
    }
    return board
}

pub fn init_game( id_mapping: &HashMap<i16, Vec<i16>>, board: &mut Hexagon::<model::Case> ) -> model::Game {


    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let day = parse_input!(input_line, i32); // the game lasts 24 days: 0-23
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nutrients = parse_input!(input_line, i32); // the base score you gain from the next COMPLETE action
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let sun = parse_input!(inputs[0], i32); // your sun points
    let score = parse_input!(inputs[1], i32); // your current score
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let opp_sun = parse_input!(inputs[0], i32); // opponent's sun points
    let opp_score = parse_input!(inputs[1], i32); // opponent's score
    let opp_is_waiting = parse_input!(inputs[2], i32); // whether your opponent is asleep until the next day
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_trees = parse_input!(input_line, i32); // the current amount of trees

    let me : model::Player = model::Player{sun:sun, score:score, is_asleep:false};
    let opp : model::Player = model::Player{sun:opp_sun, score:opp_score, is_asleep:opp_is_waiting != 0};


    for i in 0..number_of_trees as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let cell_index = parse_input!(inputs[0], i16); // location of this tree
        let size = parse_input!(inputs[1], i8); // size of this tree: 0-3
        let is_mine = parse_input!(inputs[2], i32); // 1 if this is your tree
        let is_dormant = parse_input!(inputs[3], i32); // 1 if this tree is dormant

        let mut case = board.get_mut( &id_mapping[&cell_index] ).get_value_mut();
        case.tree = size;
        case.is_mine = is_mine != 0;
        case.is_dormant = is_dormant != 0;
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_possible_moves = parse_input!(input_line, i32);

    let mut actions : Vec<model::Action> = Vec::with_capacity(number_of_possible_moves as usize);

    for i in 0..number_of_possible_moves as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let possible_move = input_line.trim_matches('\n').split(" ").collect::<Vec<_>>();
        if possible_move[0] == "GROW" {
            let grow = parse_input!(possible_move[1], i8);
            actions.push(model::Action::GROW(grow));
        }
        if possible_move[0] == "SEED" {
            let sourceIdx = parse_input!(possible_move[1], i8);
            let targetIdx = parse_input!(possible_move[1], i8);
            actions.push(model::Action::SEED(sourceIdx, targetIdx));
        }
        if possible_move[0] == "COMPLETE" {
            let cellIdx = parse_input!(possible_move[1], i8);
            actions.push(model::Action::COMPLETE(cellIdx));
        }
        if possible_move[0] == "WAIT" {
            let mut message = "";
            if possible_move.len() > 1 {
                message = possible_move[1];
            }
            actions.push(model::Action::WAIT(message.to_string()));
        }
    }

    let mut game : model::Game = model::Game{
        day:day,
        nextCompleteScore:nutrients,
        myPlayer:me,
        vsPlayer:opp,
        actions:actions
    };

    return game;


}