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
    let mut created_board : Hexagon::<model::Case> = Hexagon::<model::Case>::new(3, &model::Case{
        richness: -1
    });
    for _i in 0..number_of_cells as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let index = parse_input!(inputs[0], i16); // 0 is the center cell, the next cells spiral outwards
        let richness = parse_input!(inputs[1], i8); // 0 if the cell is unusable, 1-3 for usable cells
        created_board.get_mut( &id_mapping[&index] ).get_value_mut().richness = richness;
    }
    return created_board
}

pub fn init_game( id_mapping: &HashMap<i16, Vec<i16>>) -> model::Game {


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

    let me : model::Player = model::Player{sun:sun, score:score, is_asleep:false};
    let opp : model::Player = model::Player{sun:opp_sun, score:opp_score, is_asleep:opp_is_waiting != 0};

    let number_of_trees = parse_input!(input_line, i32); // the current amount of trees
    let mut trees : Vec<model::Tree> = Vec::with_capacity(number_of_trees as usize);
    for _i in 0..number_of_trees as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let cell_index = parse_input!(inputs[0], i16); // location of this tree
        let size = parse_input!(inputs[1], i8); // size of this tree: 0-3
        let is_mine = parse_input!(inputs[2], i32); // 1 if this is your tree
        let is_dormant = parse_input!(inputs[3], i32); // 1 if this tree is dormant
        trees.push(model::Tree{
            id_case:cell_index,
            size:size,
            coord:id_mapping[&cell_index].clone(),
            is_mine:is_mine!=0,
            is_dormant:is_dormant!=0
        });
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_possible_moves = parse_input!(input_line, i32);

    let mut completes: Vec<model::Action> = Vec::<model::Action>::new();
    let mut grows: Vec<model::Action> = Vec::<model::Action>::new();
    let mut seeds: Vec<model::Action> = Vec::<model::Action>::new();

    for _i in 0..number_of_possible_moves as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let possible_move = input_line.trim_matches('\n').split(" ").collect::<Vec<_>>();
        if possible_move[0] == "GROW" {
            let grow = parse_input!(possible_move[1], i8);
            grows.push(model::Action::GROW(grow));
        }
        if possible_move[0] == "SEED" {
            let source_idx = parse_input!(possible_move[1], i8);
            let target_idx = parse_input!(possible_move[1], i8);
            seeds.push(model::Action::SEED(source_idx, target_idx));
        }
        if possible_move[0] == "COMPLETE" {
            let cell_idx = parse_input!(possible_move[1], i8);
            completes.push(model::Action::COMPLETE(cell_idx));
        }
    }

    let actions : model::Actions = model::Actions{
        completes: completes,
        grows: grows,
        seeds: seeds,
    };

    let game : model::Game = model::Game{
        day:day,
        nutrients:nutrients,
        me:me,
        opp:opp,
        actions:actions,
        trees: trees
    };

    return game;


}