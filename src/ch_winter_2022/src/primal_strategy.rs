use crate::model;
use crate::constant;
use crate::helper;
use crate::game_impl;
use game::Strategy;
use board::Board;
use board::BoardCase;
use board2d::Board2D;
use board2d::Case2D;
use rand::Rng;
use std::rc::Rc;
use std::cmp::Ordering;


#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct PrimalStrategy {
}

impl Strategy < model::Case, board2d::Case2D<model::Case>, model::Player, board2d::Board2D<model::Case>, model::Actions, game_impl::GameImpl> for PrimalStrategy {
    fn compute_action(&self, game: &game_impl::GameImpl) -> model::Actions{
        let my_cases = game.case_of(&game.me);
        //eprintln!("my_cases = {:?}", my_cases);
    
        let my_empty_cases = game.case_empty(&my_cases);
        //eprintln!("my_empty_cases = {:?}", my_empty_cases);
        
        let my_robot_cases = game.case_robot(&my_cases);
        //eprintln!("my_robot_cases = {:?}", my_robot_cases);
        
        let my_recycler_cases = game.case_recycler(&my_cases);
        //eprintln!("my_recycler_cases = {:?}", my_recycler_cases);
        
        let opp_cases = game.case_of(&game.opp);
        //eprintln!("opp_cases = {:?}", opp_cases);
        
        let opp_robot_cases = game.case_robot(&opp_cases);
        //eprintln!("opp_robot_cases = {:?}", my_robot_cases);
    
        let neutral_cases = game.case_of(&model::NEUTRAL);
    
        // Build recycleur
        let mut build_actions = game.action_build_opportunities(&my_empty_cases);

        build_actions.sort_by(|a, b| {
            if let model::Action::BUILD(a_x, a_y) = &a {
                if let model::Action::BUILD(b_x, b_y) = &b {
                    let a_value = helper::compute_recycle_porentiel(game.board.get(&vec![*a_x, *a_y]), &game.board);
                    let b_value = helper::compute_recycle_porentiel(game.board.get(&vec![*b_x, *b_y]), &game.board);
                    return a_value.cmp(&b_value)
                }
                return Ordering::Equal
            }
            return Ordering::Equal
        });
        //eprintln!("build_actions = {:?}", build_actions);

        // Build robots
        // Todo manage spawn value
        let mut spawn_actions = game.action_spawn_opportunities(&my_empty_cases);

        // Move robot
        let (attack, defence) = my_robot_cases.split_at(my_robot_cases.len()/3);
        // Move Robot Attack
        let mut move_attack_actions : Vec<model::Action> = attack.iter().map(|case| { 
            let dest = &opp_cases[rand::thread_rng().gen_range(0..opp_cases.len())];
            model::Action::MOVE(case.get_value().units/2+1,case.position()[0], case.position()[1], dest.position()[0], dest.position()[1]) }).collect();
        // Move Robot Attack
        let mut move_defence_actions : Vec<model::Action> = defence.iter().map(|case| { 
            let dest = &neutral_cases[rand::thread_rng().gen_range(0..neutral_cases.len())];
            // Todo remove on list selected case
            model::Action::MOVE(case.get_value().units/2+1,case.position()[0], case.position()[1], dest.position()[0], dest.position()[1]) }).collect();

                // Select actions
        let mut actions: Vec<model::Action> = Vec::<model::Action>::new();

        let mut matter = game.me.matter;
        
        // Build and spawn
        if game.turn > 2 && opp_robot_cases.len() > 2 && my_recycler_cases.len() < my_robot_cases.len() / 3 && matter >= constant::BUILD_COST && !build_actions.is_empty() {
            actions.push(build_actions.pop().unwrap());
            matter -= constant::BUILD_COST;
            eprintln!("Build {:?}", actions.last().unwrap());
        }
        while matter > 0 && !spawn_actions.is_empty() {
            actions.push(spawn_actions.pop().unwrap());
            eprintln!("spawn {:?}", actions.last().unwrap());
            matter -= constant::SPAWN_COST;
        }

        // Move
        actions.append(&mut move_attack_actions);
        actions.append(&mut move_defence_actions);



        // If nothing
        if actions.is_empty() {
            actions.push(model::Action::WAIT());
        }

        return model::Actions{actions : actions}
    }
}
