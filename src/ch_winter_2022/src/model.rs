
use std::rc::Rc;
use board2d::Board2D;
use logger;

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub enum Action {
    MOVE(i16, i16, i16, i16, i16),
    BUILD(i16, i16),
    SPAWN(i16, i16, i16),
    WAIT(),
    MESSAGE(String),
}

#[derive(Eq,PartialEq,Debug,Clone,Hash,Copy)]
pub enum CaseType {
    GRASS(),
    RECYCLER(),
    EMPTY(),
    ROBOT(i16)
}

impl Action {
    pub fn to_string(&self) -> String {
        match self
        {
            Action::MOVE(amount, from_x, from_y, to_x, to_y) => format!("MOVE {} {} {} {} {}", amount, from_x, from_y, to_x, to_y), 
            Action::BUILD(x, y) => format!("BUILD {} {}", x, y),
            Action::SPAWN(amount, x, y) => format!("SPAWN {} {} {}", amount, x, y),
            Action::WAIT() => format!("WAIT"),
            Action::MESSAGE(text) => format!("MESSAGE {}", text),
        }
    }
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Actions {
    pub actions: Vec<Action>,
}
impl Actions {
    pub fn print(&self) {
        logger::output(self.actions.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(";"));
    }
}

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Case {
    pub scrap_amount: i16,
    pub owner_id: i16,
    pub case_type: CaseType,
    pub units: i16,
    pub can_build: bool,
    pub can_spawn: bool,
    pub in_range_of_recycler: bool,
}


#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Player {
    pub id: i16,
    pub matter: i16,
}

pub const NEUTRAL : Player = Player{id: -1, matter: 0};



#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Params {
}
