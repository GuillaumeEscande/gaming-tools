use board::LinearHexagon;
use board::Board;
use crate::model;

pub fn extract_complete(
    board: &LinearHexagon::<model::Case>,
    game : &mut model::Game,
    params : &model::Params,
    better_cases: &Vec<i16> ) -> Option<model::Action> {
        // 1 : recherche d'un cmplete en priorité
        eprintln!("-- extract_complete");
        if game.actions.completes.len() > 0 {
            if game.me.sun >= params.seed_cost + 4 {
                eprintln!("-- complete in better_case ? and can seed after ?");
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
            }
            eprintln!("-- Run first complete");
            if game.me.sun >= 4 && game.day > 19 {
                return Some(game.actions.completes.remove(0));
            } else {
                eprintln!("---- Not enought points");
            }
        } else {
            eprintln!("---- No COMPLETE");
        }
        return None;
}