
#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub enum Action {
    WAIT(String),
    SEED(i16, i16),
    GROW(i16),
    COMPLETE(i16),
}

impl Action {
    pub fn print(&self) {
        match self
        {
            Action::WAIT(message) => println!("WAIT {}", message),
            Action::SEED(source, target) => println!("SEED {} {}", source, target),
            Action::GROW(size) => println!("GROW {}", size),
            Action::COMPLETE(val) => println!("COMPLETE {}", val)
        }
    }
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Actions {
    pub completes: Vec<Action>,
    pub grows: Vec<Action>,
    pub seeds: Vec<Action>,
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Case {
    pub richness: i16
}

use std::vec::Vec;

pub fn get_id_mapping() -> Vec<Vec<i16>> {
    let mut id_maping = Vec::with_capacity(37);

    id_maping.push(vec!( 0,  0,  0));
    id_maping.push(vec!( 1, -1,  0));
    id_maping.push(vec!( 1,  0, -1));
    id_maping.push(vec!( 0,  1, -1));
    id_maping.push(vec!(-1,  1,  0));
    id_maping.push(vec!(-1,  0,  1));
    id_maping.push(vec!( 0, -1,  1));
    id_maping.push(vec!( 2, -2,  0));
    id_maping.push(vec!( 2, -1, -1));
    id_maping.push(vec!( 2,  0, -2));
    id_maping.push(vec!( 1,  1, -2));
    id_maping.push(vec!( 0,  2, -2));
    id_maping.push(vec!(-1,  2, -1));
    id_maping.push(vec!(-2,  2,  0));
    id_maping.push(vec!(-2,  1,  1));
    id_maping.push(vec!(-2,  0,  2));
    id_maping.push(vec!(-1, -1,  2));
    id_maping.push(vec!( 0, -2,  2));
    id_maping.push(vec!( 1, -2,  1));
    id_maping.push(vec!( 3, -3,  0));
    id_maping.push(vec!( 3, -2, -1));
    id_maping.push(vec!( 3, -1, -2));
    id_maping.push(vec!( 3,  0, -3));
    id_maping.push(vec!( 2,  1, -3));
    id_maping.push(vec!( 1,  2, -3));
    id_maping.push(vec!( 0,  3, -3));
    id_maping.push(vec!(-1,  3, -2));
    id_maping.push(vec!(-2,  3, -1));
    id_maping.push(vec!(-3,  3,  0));
    id_maping.push(vec!(-3,  2,  1));
    id_maping.push(vec!(-3,  1,  2));
    id_maping.push(vec!(-3,  0,  3));
    id_maping.push(vec!(-2, -1,  3));
    id_maping.push(vec!(-1, -2,  3));
    id_maping.push(vec!( 0, -3,  3));
    id_maping.push(vec!( 1, -3,  2));
    id_maping.push(vec!( 2, -3,  1));

    return id_maping;
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Tree {
    pub id_case: i16,
    pub size: i16,
    pub is_mine: bool,
    pub is_dormant: bool
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Player {
    pub sun: i32,
    pub score: i32,
    pub is_asleep: bool
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Game {
    pub day: i32,
    pub nutrients: i32,
    pub me: Player,
    pub opp: Player,
    pub actions: Actions,
    pub trees: Vec<Option<Tree>>
}

impl Game{
    pub fn print(&self){
        eprintln!("Day : {}", self.day);
        eprintln!("Nutrients : {}", self.nutrients);
        eprintln!("Me : ");
        eprintln!("  sun : {}", self.me.sun);
        eprintln!("  score : {}", self.me.score);
        eprintln!("  is_asleep : {}", self.me.is_asleep);
        eprintln!("Opp : ");
        eprintln!("  sun : {}", self.me.sun);
        eprintln!("  score : {}", self.me.score);
        eprintln!("  is_asleep : {}", self.me.is_asleep);
        eprintln!("Actions :");
        eprintln!("  Completes :");
        for action in &self.actions.completes {
            eprintln!("    - {:?}", action);
        }
        eprintln!("  Grows :");
        for action in &self.actions.grows {
            eprintln!("    - {:?}", action);
        }
        eprintln!("  Seeds :");
        for action in &self.actions.seeds {
            eprintln!("    - {:?}", action);
        }
        eprintln!("Trees :");
        for tree in &self.trees {
            match tree {
                Some(treev) => {
                    eprintln!("  - id = {}", treev.id_case);
                    eprintln!("    size = {}", treev.size);
                    eprintln!("    is_mine = {}", treev.is_mine);
                    eprintln!("    is_dormant = {}", treev.is_dormant);
                }
                None    => {},
            }
        }
    }
}