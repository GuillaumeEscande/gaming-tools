use std::collections::HashMap;
use board::Hexagon;
use crate::model;
use board::Board;


pub fn choose1( used_sun: &mut i32, _id_mapping: &HashMap<i16, Vec<i16>>, _board: &Hexagon::<model::Case>, game : &mut model::Game ) -> model::Action {

    // First action - find COMPLETE
    if game.actions.completes.len() > 0 {
        let action = game.actions.completes.remove(0);
        *used_sun += 1;
        return action;
    }


    // Second action - SEED directly on the sun
    let mut seed : Option<usize> = None;
    for i in 0..game.actions.seeds.len() {
        match &game.actions.seeds[i] {
            &model::Action::SEED(_, target) => {
                if target < 6 {
                    seed = Some(i);
                    break;
                }
            },
            _ => ()
        }
    }
    if seed.is_some() {
        let action = game.actions.seeds.remove(seed.unwrap());
        *used_sun += 1;
        return action;
    }

    // Third Action - growing tree

    return game.actions.grows.pop().unwrap();

}
