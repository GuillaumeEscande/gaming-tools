use std::collections::HashMap;
use std::collections::LinkedList;

use board::Hexagon;
use crate::model;
use board::Board;
use std::rc::Rc;
use graph::astar::resolve_astar;


pub fn choose1( used_sun: &mut i32, _id_mapping: &HashMap<i16, Vec<i16>>, board: &Hexagon::<model::Case>, game : &mut model::Game ) -> model::Action {

    eprintln!("-- Find COMPLETE");
    // First action - find COMPLETE
    if game.actions.completes.len() > 0 {
        let action = game.actions.completes.remove(0);
        *used_sun += 1;
        return action;
    } else {
        eprintln!("---- No COMPLETE");
    }
    

    eprintln!("-- Find SEED");
    // Second action - SEED directly on the sun
    let mut seed : Option<usize> = None;
    for i in 0..game.actions.seeds.len() {
        match &game.actions.seeds[i] {
            &model::Action::SEED(origin, target) => {
                if origin != target {
                    eprintln!("seed {}-{}", origin, target);
                    seed = Some(i);
                }
            },
            _ => ()
        }
    }
    if seed.is_some() {
        let action = game.actions.seeds.remove(seed.unwrap());
        *used_sun += 1;
        eprintln!("seed {:?}", action);
        return action;
    } else {
        eprintln!("---- No SEED");
    }

    // Third Action - growing tree
    if game.actions.grows.len() > 1 {
        eprintln!("-- Last chance : Last Action");
        let action = game.actions.grows.pop().unwrap();
        *used_sun += 1;
        return action;
    }
   return  model::Action::WAIT("Je ne sais pas quoi faire".to_string());

}
