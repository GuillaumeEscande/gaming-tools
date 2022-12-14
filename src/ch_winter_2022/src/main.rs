mod model;
mod init;
mod constant;
mod tools;
mod finder;

use rand::Rng;
use std::rc::Rc;
use board::Board;
use board::BoardCase;


fn main() {
    // Init data
    let size = init::init_size();
    let mut cpt_turn = 0;

    // game loop
    loop {
        // Build game
        let init = init::init_game(size, cpt_turn);
        let game = init.0;
        let board = init.1;

        eprintln!("---------- START TURN ----------");
        game.print();

        let my_cases : Vec<Rc<board2d::Case2D<model::Case>>> = board.to_linear().iter().map(|m| Rc::clone(m)).filter(|case| case.get_value().owner_id == game.me.id).collect();
        //eprintln!("my_cases = {:?}", my_cases);

        let my_empty_cases : Vec<Rc<board2d::Case2D<model::Case>>> = my_cases.iter().map(|m|  Rc::clone(m)).filter(|case| case.get_value().case_type == model::CaseType::EMPTY()).collect();
        //eprintln!("my_empty_cases = {:?}", my_empty_cases);
        
        let my_robot_cases : Vec<Rc<board2d::Case2D<model::Case>>> = my_cases.iter().map(|m|  Rc::clone(m)).filter(|case| match &case.get_value().case_type { model::CaseType::ROBOT(_) => true, other => false,}).collect();
        eprintln!("my_robot_cases = {:?}", my_robot_cases);
        
        let my_recycler_cases : Vec<Rc<board2d::Case2D<model::Case>>> = my_cases.iter().map(|m|  Rc::clone(m)).filter(|case| case.get_value().case_type == model::CaseType::RECYCLER()).collect();
        //eprintln!("my_recycler_cases = {:?}", my_recycler_cases);
        
        let opp_cases : Vec<Rc<board2d::Case2D<model::Case>>> = board.to_linear().iter().map(|m| Rc::clone(m)).filter(|case| case.get_value().owner_id == game.opp.id).collect();
        eprintln!("opp_cases = {:?}", opp_cases);
        
        let opp_robot_cases : Vec<Rc<board2d::Case2D<model::Case>>> = opp_cases.iter().map(|m|  Rc::clone(m)).filter(|case| match &case.get_value().case_type { model::CaseType::ROBOT(_) => true, other => false,}).collect();
        //eprintln!("opp_robot_cases = {:?}", my_robot_cases);

        let neutral_cases : Vec<Rc<board2d::Case2D<model::Case>>> = board.to_linear().iter().map(|m| Rc::clone(m)).filter(|case| case.get_value().owner_id == model::NEUTRAL.id).collect();




        eprintln!("------- LIST ALL UNITARY ACTIONS -------");
        // Build recycleur
        let mut build_actions : Vec<model::Action> = my_empty_cases.iter().map(|case| { model::Action::BUILD(case.position()[0], case.position()[1]) }).collect();

        // Build robots
        // Todo manage spawn value
        let mut spawn_actions : Vec<model::Action> = my_empty_cases.iter().map(|case| { model::Action::SPAWN(1, case.position()[0], case.position()[1]) }).collect();

        // Move robot
        let (attack, defence) = my_robot_cases.split_at(my_robot_cases.len()/3);
        // Move Robot Attack
        let mut move_attack_actions : Vec<model::Action> = attack.iter().map(|case| { 
            let dest = &opp_cases[rand::thread_rng().gen_range(0..opp_cases.len())];
            model::Action::MOVE(case.get_value().units/2+1,case.position()[0], case.position()[1], dest.position()[0], dest.position()[1]) }).collect();
        // Move Robot Attack
        let mut move_defence_actions : Vec<model::Action> = defence.iter().map(|case| { 
            let dest = &neutral_cases[rand::thread_rng().gen_range(0..neutral_cases.len())];
            model::Action::MOVE(case.get_value().units/2+1,case.position()[0], case.position()[1], dest.position()[0], dest.position()[1]) }).collect();
    

        
        // Move Robot Defence

        eprintln!("-------- CHOOSE ACTIONS --------");
        // Select actions
        let mut actions: Vec<model::Action> = Vec::<model::Action>::new();

        let mut matter = game.me.matter;
        
        // Build and spawn
        if opp_robot_cases.len() > 2 && my_robot_cases.len() < 5 && my_recycler_cases.len() < my_robot_cases.len() / 4 && matter >= constant::BUILD_COST && !build_actions.is_empty() {
            actions.push(build_actions[rand::thread_rng().gen_range(0..build_actions.len())].clone());
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


        eprintln!("-------- PRINT ACTIONS ---------");
        tools::print_actions(actions);

        eprintln!("---------- END ----------");
        cpt_turn = game.turn;

    }
}
