
use board::Board;
use board::BoardCase;
use linearhexagon::LinearHexagon;
use linearhexagon::LinearHexagonCase;
use game::Game;
use std::rc::Rc;
use std::collections::LinkedList;
use crate::model;



pub fn compute_actions(game : & model::TreeGame) -> LinkedList< model::Action >{
    let mut actions = LinkedList::new();

    // WAIT
    actions.push_back(model::Action::WAIT("".to_string()));

    // SEED
    for i in 0..game.trees.len() {
        let origin_case = game.board.get(&vec!(i as i16));
        match &game.trees[i] {
            Some(tree) => {
                if tree.is_mine {
                    for j in 0..game.board.size()[0] {
                        if game.trees[j].is_none() {
                            let case = game.board.get(&vec!(j as i16));
                            if case.get_value().richness > 0 && game.board.distance(&origin_case, &case) <= tree.size {
                                actions.push_back(model::Action::SEED(i as i16, j as i16));
                            }
                        }
                    }
                }
            },
            _ => ()
        }
    }

    // COMPLETE
    for i in 0..game.trees.len() {
        match &game.trees[i] {
            Some(tree) => {
                if tree.size == 3 {
                    actions.push_back(model::Action::COMPLETE(i as i16));
                }
            },
            _ => ()
        }
    }

    return actions;
}
