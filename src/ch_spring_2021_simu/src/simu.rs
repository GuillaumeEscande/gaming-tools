
use board::Board;
use board::BoardCase;
use linearhexagon::LinearHexagon;
use linearhexagon::LinearHexagonCase;
use game::Game;
use std::rc::Rc;
use std::collections::LinkedList;
use crate::model;



pub fn simulate_turn(game : &model::TreeGame, action: &model::Action) -> Rc<model::TreeGame>{
    eprintln!("simulate_turn ");

    //change player
    //compute score
    //Update plateau


    let mut new_game : Rc<model::TreeGame> = Rc::new(model::TreeGame{
        board: Rc::clone(&game.board),
        day:game.day + 1,
        nutrients:game.nutrients + 1,
        me:game.opp.clone(),
        opp:game.me.clone(),
        trees: game.trees.clone(),
    });

    return new_game;
}
