use crate::linear_hexagon;
use crate::model;
use crate::constant;
use board::Board;
 

pub fn get_nearest_case_to_seed (
    board: &linear_hexagon::LinearHexagon::<model::Case>,
    game : &mut model::Game,
    params : &model::Params,
    better_cases: &Vec<i16> ) -> Option<model::Action> {

        let mut better_seed_action : i16 = -1;
        let mut nearest_target_distance : i16 = i16::MAX;

        for i in 0..better_cases.len() {
            if game.trees[better_cases[i] as usize].is_none() {

                let case_indice = better_cases[i];

                let dest = board.get(&vec!(case_indice));
                // Si pas d'arbre
                // Pour tous les seed possible on prend le plus court
                for i in 0..game.actions.seeds.len() {
                    match &game.actions.seeds[i] {
                        &model::Action::SEED(_, target) => {
                            let source = board.get(&vec!(target));
                            //let way = graph::resolve_astar(board, &source, &dest);
                            let distance = board.distance(&source, &dest);
                            if distance < nearest_target_distance {
                                nearest_target_distance = distance;
                                better_seed_action = i as i16;
                                //eprintln!("---- nearest : {} of {}", target, case_indice);
                            }
                        },
                        _ => ()
                    }
                }
            }
        }
        if better_seed_action > -1 {
            let action = game.actions.seeds.remove(better_seed_action as usize);
            return Some(action);
        }
        return None;
}


pub fn extract_seed(
    board: &linear_hexagon::LinearHexagon::<model::Case>,
    game : &mut model::Game,
    params : &model::Params,
    better_cases: &Vec<i16> ) -> Option<model::Action> {

    eprintln!("-- Seed better_case");
    if game.me.sun >= params.seed_cost /*&& nb_tree[0] < 3 && nb_tree[1] < 5*/ && game.day > 2 && game.day < 10 {
        eprintln!("---- find the nearest case to seed");
        let mut better_seed_action : i16 = -1;
        let mut nearest_target_distance : i16 = i16::MAX;
        for i in 0..better_cases.len() {
            eprintln!("---- find the nearest case to seed");
            if game.trees[better_cases[i] as usize].is_none() {
                let case_indice = better_cases[i];
                eprintln!("---- No tree on {}", case_indice);
                let dest = board.get(&vec!(case_indice));
                // Si pas d'arbre
                // Pour tous les seed possible on prend le plus court
                for i in 0..game.actions.seeds.len() {
                    match &game.actions.seeds[i] {
                        &model::Action::SEED(origin, target) => {
                            let source = board.get(&vec!(target));
                            //let way = graph::resolve_astar(board, &source, &dest);
                            let distance = board.distance(&source, &dest);
                            if distance < nearest_target_distance {
                                nearest_target_distance = distance;
                                better_seed_action = i as i16;
                                //eprintln!("---- nearest : {} of {}", target, case_indice);
                            }
                        },
                        _ => ()
                    }
                }
            }
        }
        if better_seed_action > -1 {
            let action = game.actions.seeds.remove(better_seed_action as usize);
            return Some(action);
        }
    } else {
        eprintln!("---- Seed to expensive {}", params.seed_cost);
    }
    return None;
}
