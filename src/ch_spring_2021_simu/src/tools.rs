
use board::Board;
use board::BoardCase;
use linearhexagon::LinearHexagon;
use linearhexagon::LinearHexagonCase;
use game::Game;
use std::rc::Rc;
use std::collections::LinkedList;
use crate::model;



pub fn reset_board(game: &mut Rc<model::TreeGame>){
    let day = game.day;

    let game_mut : &mut model::TreeGame = Rc::get_mut(game).unwrap();
    let board : &mut linearhexagon::LinearHexagon<model::Case> = Rc::get_mut(&mut game_mut.board).unwrap();
    // Reset shadow
    for i in 0..board.size()[0] {
        let case : &mut linearhexagon::LinearHexagonCase<model::Case> = Rc::get_mut(board.get_mut(&vec!(i as i16))).unwrap();
        let mut case_content : &mut model::Case = Rc::get_mut(case.get_mut_value()).unwrap();
        case_content.shadow = 0;
        case_content.tree = None;
    }
}

pub fn update_tree(game: &mut Rc<model::TreeGame>){

    let game_mut : &mut model::TreeGame = Rc::get_mut(game).unwrap();
    let trees = &game_mut.trees;
    let board : &mut linearhexagon::LinearHexagon<model::Case> = Rc::get_mut(&mut game_mut.board).unwrap();
    for i in 0..game_mut.trees.len() {
        match &game_mut.trees[i] {
            Some(tree) => {
                let case : &mut linearhexagon::LinearHexagonCase<model::Case> = Rc::get_mut(board.get_mut(&vec!(i as i16))).unwrap();
                let mut case_content : &mut model::Case = Rc::get_mut(case.get_mut_value()).unwrap();
                case_content.tree = Some(Rc::clone(&tree));
            },
            _ => {}
        }
    }
}

pub fn update_shadow(game: &mut Rc<model::TreeGame>){
    let game_mut : &mut model::TreeGame = Rc::get_mut(game).unwrap();
    let day = game_mut.day;
    let direction = (day % 6);

    // Reset shadow
    for i in 0..game_mut.board.size()[0] {
        let case_content : &model::Case = game_mut.board.get(&vec!(i as i16)).get_value();
        match &case_content.tree {
            Some(tree) => {
                for i in 0..tree.size {
                    
                }
            },
            _ => {}
        }
    }
}
