use crate::linear_hexagon;
use board::Board;
use crate::model;
use crate::complete;
use crate::constant;

pub fn extract_complete(
    board: &linear_hexagon::LinearHexagon::<model::Case>,
    game : &mut model::Game,
    params : &model::Params,
    better_cases: &Vec<i16> ) -> Option<model::Action> {
        // 1 : recherche d'un cmplete en priorité
/*        
        for complete in &game.actions.completes {
            match complete {
                model::Action::COMPLETE(case) => {
                    if better_cases.contains(&case) {
                        eprintln!("---- complete on better_case {}", case);
                        complete.print();
                        return Some(complete.clone());
                    }
                },
                _ => {}
            }
        }

        eprintln!("-- extract_complete");
                eprintln!("-- complete in better_case ? and can seed after ?");

            eprintln!("-- Run first complete");
            if game.me.sun >= 4 && game.day > 19 {
                return Some(game.actions.completes.remove(0));
            } else {
                eprintln!("---- Not enought points");
            }
        } else {
            eprintln!("---- No COMPLETE");
        }*/
        return None;
}
