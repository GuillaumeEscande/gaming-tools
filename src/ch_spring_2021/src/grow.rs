use board::LinearHexagon;
use board::Board;
use crate::model;

pub fn extract_seed_to_grow(
    board: &LinearHexagon::<model::Case>,
    game : &mut model::Game,
    params : &model::Params,
    better_cases: &Vec<i16> ) -> Option<model::Action> {

    // 1 : recherche d'un grow car une graine n'aporte rien
    if game.actions.grows.len() > 1 {
        eprintln!("-- Grow seed");
        if game.me.sun >= params.grow_cost[0] {
            for grow in &game.actions.grows {
                match grow {
                    model::Action::GROW(case) => {
                        if game.trees[*case as usize].as_ref().unwrap().size == 0 {
                            return Some(grow.clone());
                        }
                    },
                    _ => {}
                }
            }
        } else {
            eprintln!("---- Not enought points");
        }
    }
    return None
}


pub fn extract_grow(
    board: &LinearHexagon::<model::Case>,
    game : &mut model::Game,
    params : &model::Params,
    better_cases: &Vec<i16> ) -> Option<model::Action> {

    // Third Action - growing tree
    if game.actions.grows.len() > 1 && ( game.day < 4 || game.day > 15 || params.nb_tree[0] > 2 || params.nb_tree[1] > 2) {
        let mut grow_value = 0;
        let mut grow_id = 0;
        for i in 0..game.actions.grows.len() {
            match &game.actions.grows[i] {
                model::Action::GROW(case) => {
                    if game.trees[*case as usize].as_ref().unwrap().size > grow_value {
                        grow_value = game.trees[*case as usize].as_ref().unwrap().size;
                        grow_id = i;
                    }
                },
                _ => {}
            }
        }
        eprintln!("-- Last chance : Last Action"); if game.day > 15 {
            let action = game.actions.grows.remove(grow_id);
            return Some(action);
        } else {
            let action = game.actions.grows.remove(0);
            return Some(action);
        }
    }
    return None;
}