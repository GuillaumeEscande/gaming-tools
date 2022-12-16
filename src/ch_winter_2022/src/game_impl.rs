
use crate::model;
use game::Game;
use game::Strategy;
use board::BoardCase;
use board::Board;
use board2d::Board2D;
use board2d::Case2D;
use std::rc::Rc;


#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct GameImpl {
    pub board: board2d::Board2D::<model::Case>,
    pub turn: i16,
    pub me: model::Player,
    pub opp: model::Player
}

impl GameImpl {
    pub fn print(&self){
        eprintln!("Turn : {}", self.turn);
        eprintln!("Me : ");
        eprintln!("  matter : {}", self.me.matter);
        eprintln!("Opp : ");
        eprintln!("  matter : {}", self.me.matter);
    }

    pub fn case_of(&self, player: &model::Player) -> Vec<Rc<board2d::Case2D<model::Case>>>{
        return self.board.to_linear().iter().map(|m| Rc::clone(m)).filter(|case| case.get_value().owner_id == player.id).collect();
    }

    pub fn case_empty(&self, cases: &Vec<Rc<board2d::Case2D<model::Case>>>) -> Vec<Rc<board2d::Case2D<model::Case>>>{
        return cases.iter().map(|m|  Rc::clone(m)).filter(|case| match &case.get_value().case_type { model::CaseType::EMPTY() => true, _other => false,}).collect();
    }

    pub fn case_robot(&self, cases: &Vec<Rc<board2d::Case2D<model::Case>>>) -> Vec<Rc<board2d::Case2D<model::Case>>>{
        return cases.iter().map(|m|  Rc::clone(m)).filter(|case| match &case.get_value().case_type { model::CaseType::ROBOT(_) => true, _other => false,}).collect();
    }

    pub fn case_recycler(&self, cases: &Vec<Rc<board2d::Case2D<model::Case>>>) -> Vec<Rc<board2d::Case2D<model::Case>>>{
        return cases.iter().map(|m|  Rc::clone(m)).filter(|case| match &case.get_value().case_type { model::CaseType::RECYCLER() => true, _other => false,}).collect();
    }

    pub fn action_build_opportunities(&self, empty_cases: &Vec<Rc<board2d::Case2D<model::Case>>>) -> Vec< model::Action >{
        return empty_cases.iter().map(|case| { model::Action::BUILD(case.position()[0], case.position()[1]) }).collect();
    }

    pub fn action_spawn_opportunities(&self, empty_cases: &Vec<Rc<board2d::Case2D<model::Case>>>) -> Vec< model::Action >{
        // Todo manage spawn value
        return empty_cases.iter().map(|case| { model::Action::SPAWN(1, case.position()[0], case.position()[1]) }).collect();
    }

    pub fn action_move_opportunities(&self, robot_cases: &Vec<Rc<board2d::Case2D<model::Case>>>) -> Vec< model::Action >{
        let mut move_actions = Vec::new();
        robot_cases.iter().for_each(|case| { 
            self.board.neighbors(&case).iter().for_each(|neighbor| {
                // Todo manage move value
                move_actions.push(model::Action::MOVE(case.get_value().units/2+1,case.position()[0], case.position()[1], neighbor.position()[0], neighbor.position()[1]));
            });
        });
        return move_actions;
    }
}

impl game::Game < model::Case, board2d::Case2D<model::Case>, model::Player, board2d::Board2D<model::Case>, model::Actions>  for GameImpl {

    fn board(&self) -> &board2d::Board2D<model::Case>{
        return &self.board;
    }

    fn list_actions(&self) -> Vec< model::Actions >{

        let my_cases = self.case_of(&self.me);
        let my_empty_cases = self.case_empty(&my_cases);

        let build_actions = self.action_build_opportunities(&my_empty_cases);
        let spawn_actions = self.action_spawn_opportunities(&my_empty_cases);
        let move_actions = self.action_move_opportunities(&my_empty_cases);

        let mut actions = Vec::<model::Actions>::new();

        build_actions.iter().for_each(|action|{ actions.push(model::Actions{actions: vec![action.clone()]}) });
        spawn_actions.iter().for_each(|action|{ actions.push(model::Actions{actions: vec![action.clone()]}) });
        move_actions.iter().for_each(|action|{ actions.push(model::Actions{actions: vec![action.clone()]}) });

        return actions;
    }

    fn apply_action(&self, action: &model::Actions) -> Rc<GameImpl>{
        let mut new_game = self.clone();
        new_game.turn += 1;

        action.actions.iter().for_each(|action|{
            match action {
                model::Action::MOVE(value, from_x, from_y, to_x, to_y) => {
                    
                },
                model::Action::BUILD(x, y) => {
                    let board = &mut new_game.board();
                    let case = &mut board.get(&vec![*x, *y]);
                    let case_value = &mut case.get_value();
                    if case_value.owner_id == new_game.me.id {
                        if let model::CaseType::EMPTY() = case_value.case_type {
                            case_value.case_type == model::CaseType::RECYCLER();
                        }
                    }
                   
                },
                model::Action::SPAWN(value, x, y) => {
                    
                },
                model::Action::WAIT() => {
                    
                },
                model::Action::MESSAGE(_) => {
                    
                }
            }
        });

        // Compute matter

        return Rc::new(new_game);
    }

    fn score(&self) -> i64{
        return self.board.to_linear().iter().filter(|case| case.get_value().owner_id == self.me.id).count() as i64;
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
