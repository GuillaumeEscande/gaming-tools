use std::collections::HashMap;
use std::collections::LinkedList;

use board::LinearHexagon;
use crate::model;
use board::Board;
use std::rc::Rc;
use graph::resolve_astar;
use crate::init;

pub fn choose1(
    board: &LinearHexagon::<model::Case>,
    game : &mut model::Game,
    better_cases: &Vec<i16> ) -> () {


    let mut nb_tree = vec!(0, 0, 0, 0);
    let mut my_tree : Vec<model::Tree> = Vec::with_capacity(game.trees.len());
    for tree_opt in &game.trees {
        match tree_opt{
            Some(tree) => {
                if tree.is_mine {
                    nb_tree[tree.size as usize] += 1;
                    my_tree.push(tree.clone());
                }
            },
            None    => {}
        }
    }
    let seed_cost = nb_tree[0];

    let mut better_cases_tree : Vec<bool> = Vec::with_capacity(better_cases.len());
    for better_case in better_cases {
        match game.trees[*better_case as usize]{
            Some(_) => {
                better_cases_tree.push(true);
            },
            None    => {
                eprintln!("TEST 01  case {} = {}", better_case, board.get(&vec!(*better_case)).get_value().richness);
                if board.get(&vec!(*better_case)).get_value().richness == 0 {
                    better_cases_tree.push(true);                    
                } else {
                    better_cases_tree.push(false);
                }
            }
        }
    }

    let grow_cost = vec!(1 + nb_tree[1], 3 + nb_tree[1], 7 + nb_tree[1], 0);

    eprintln!("-- CHOOSE PARAMS : ");
    eprintln!("-- - sun restant          = {}", game.me.sun);
    eprintln!("-- - better_cases         = {:?}", better_cases);
    eprintln!("-- - better_cases_tree    = {:?}", better_cases_tree);
    eprintln!("-- - seed_cost            = {}", seed_cost);
    eprintln!("-- - nb_tree              = {:?}", nb_tree);
    eprintln!("-- - grow_cost            = {:?}", grow_cost);



    // 1 : recherche d'un cmplete en priorité
    eprintln!("-- Find COMPLETE");
    if game.actions.completes.len() > 0 {
        if game.me.sun >= seed_cost + 4 {
            eprintln!("-- complete in better_case ? and can seed after ?");
            for complete in &game.actions.completes {
                match complete {
                    model::Action::COMPLETE(case) => {
                        if better_cases.contains(&case) {
                            eprintln!("---- complete on better_case {}", case);
                            complete.print();
                        }
                    },
                    _ => {}
                }
            }
        }
        eprintln!("-- Run first complete");
        if game.me.sun >= 4 {
            let action = game.actions.completes.remove(0);
            eprintln!("---- Run compete {:?}", action);
            action.print();
            return;
        } else {
            eprintln!("---- Not enought points");
        }
    } else {
        eprintln!("---- No COMPLETE");
    }



    // 1 : recherche d'un grow car une graine n'aporte rien
    if game.actions.grows.len() > 1 {
        eprintln!("-- Grow seed");
        if game.me.sun >= grow_cost[0] {
            for grow in &game.actions.grows {
                match grow {
                    model::Action::GROW(case) => {
                        if game.trees[*case as usize].as_ref().unwrap().size == 0 {
                            eprintln!("---- Run {:?}", grow);
                            grow.print();
                            return;
                        }
                    },
                    _ => {}
                }
            }
        } else {
            eprintln!("---- Not enought points");
        }
    }


    eprintln!("-- Seed better_case");
    if game.me.sun >= seed_cost {
        eprintln!("---- find the nearest case to seed");
        let mut better_seed_action : i16 = -1;
        let mut nearest_target_distance : i16 = i16::MAX;
        for i in 0..better_cases.len() {
            eprintln!("---- find the nearest case to seed");
            if !better_cases_tree[i] {
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
            eprintln!("---- Run seed {:?}", action);
            action.print();
            return;
        }
    } else {
        eprintln!("---- Seed to expensive {}", seed_cost);
    }

    // Third Action - growing tree
    if game.actions.grows.len() > 1 {
        eprintln!("-- Last chance : Last Action");
        let action = game.actions.grows.pop().unwrap();
        eprintln!("---- Run {:?}", action);
        action.print();
        return;
    }
   model::Action::WAIT("Je ne sais pas quoi faire".to_string()).print();

}
