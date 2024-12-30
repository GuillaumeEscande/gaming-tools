
use crate::model;
use game::Game;
use game::Strategy;
use board::BoardCase;
use board::Board;
use linearhexagon::LinearHexagon;
use linearhexagon::LinearHexagonCase;
use std::rc::Rc;


#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct GameImpl {
    pub board: linearhexagon::LinearHexagon::<model::Case>,
    pub turn: i16,
    pub number_of_base: i16,
    pub me: model::Player,
    pub opp: model::Player
}

impl GameImpl {
    pub fn print(&self){
        eprintln!("Turn : {}", self.turn);
    }

}

impl game::Game < model::Case, linearhexagon::LinearHexagonCase<model::Case>, model::Player, linearhexagon::LinearHexagon<model::Case>, model::Actions>  for GameImpl {

    fn board(&self) -> &linearhexagon::LinearHexagon<model::Case>{
        return &self.board;
    }

    fn list_actions(&self) -> Vec< model::Actions >{

        let mut actions = Vec::<model::Actions>::new();

        return actions;
    }

    fn apply_action(&self, action: &model::Actions) -> Rc<GameImpl>{
        let mut new_game = self.clone();

        return Rc::new(new_game);
    }

    fn score(&self) -> i64{
        return 0;
    }

    fn me(&self) -> &model::Player{
        return &self.me;
    }

    fn opposite(&self) -> Rc<GameImpl>{
        let mut new_game = self.clone();
        new_game.me = self.opp.clone();
        new_game.opp = self.me.clone();
        return Rc::new(new_game);
    }

    fn is_terminal(&self) -> bool{
        //TODO
        return false;
    }

}
