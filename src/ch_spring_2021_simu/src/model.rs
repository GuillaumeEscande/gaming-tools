use board::Board;
use linearhexagon::LinearHexagon;
use game::Game;
use std::rc::Rc;
use std::collections::LinkedList;
use crate::actions;

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

#[derive(Eq,PartialEq,Debug,Clone)]
pub struct Case {
    pub richness: i16,
    pub shadow: i16,
    pub tree: Option<Rc<Tree>>
}

#[derive(Eq,PartialEq,Debug,Clone)]
pub struct Tree {
    pub size: i16,
    pub is_mine: bool,
    pub is_dormant: bool
}

#[derive(Eq,PartialEq,Debug,Clone)]
pub struct Player {
    pub sun: i16,
    pub score: i16,
    pub is_asleep: bool
}

#[derive(Eq,PartialEq,Debug,Clone)]
pub struct TreeGame {
    pub board: Rc<linearhexagon::LinearHexagon<Case>>,
    pub day: i16,
    pub nutrients: i16,
    pub me: Player,
    pub opp: Player,
    pub trees: Vec<Option<Rc<Tree>>>,
}

#[derive(Eq,PartialEq,Debug,Clone)]
pub struct Params {
    pub nb_tree: Vec<i16>,
    pub my_trees: Vec<Rc<Tree>>,
    pub seed_cost: i16,
    pub grow_cost: Vec<i16>,
}


impl game::Game<Case, linearhexagon::LinearHexagonCase<Case>, Player, linearhexagon::LinearHexagon<Case>, Action> for TreeGame{

    fn board(&self) -> Rc<linearhexagon::LinearHexagon<Case>>{
        return Rc::clone(&self.board);
    }
    fn actions(&self) -> LinkedList< Action >{
        return actions::compute_actions(&self);
    }
    fn score(&self) -> i64{
        // TODO
        return 0;
    }
    fn is_terminal(&self) -> bool{
        // TODO
        return true;
    }
    fn apply(&self, action: &Action) -> Rc<Self>{
        // TODO
        return Rc::new(self.clone());
    }
    fn me(&self) -> &Player{
        return &self.me;
    }
}

type TreeGameSolvable = solver::GameSolvable<Case, linearhexagon::LinearHexagonCase<Case>, Player, linearhexagon::LinearHexagon<Case>, Action, TreeGame>;
