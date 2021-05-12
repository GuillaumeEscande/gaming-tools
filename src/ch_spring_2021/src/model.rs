
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

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Tree {
    pub id_case: i16,
    pub size: i16,
    pub is_mine: bool,
    pub is_dormant: bool
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Player {
    pub sun: i16,
    pub score: i16,
    pub is_asleep: bool
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Game {
    pub day: i16,
    pub nutrients: i16,
    pub me: Player,
    pub opp: Player,
    pub actions: Actions,
    pub trees: Vec<Option<Tree>>

}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Params {
    pub nb_tree: Vec<i16>,
    pub my_tree: Vec<Tree>,
    pub seed_cost: i16,
    pub grow_cost: Vec<i16>,
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