
use std::rc::Rc;
use linearhexagon::LinearHexagon;
use logger;

#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub enum Action {
    BEACON(i16, i16),
    LINE(i16, i16, i16),
    WAIT(),
    MESSAGE(String),
}

#[derive(Eq,PartialEq,Debug,Clone,Hash,Copy)]
pub enum CaseType {
    EMPTY(),
    EGGS(i16),
    CRISTAL(i16),
    BASE()
}

impl Action {
    pub fn to_string(&self) -> String {
        match self
        {
            Action::BEACON(index, strength) => format!("BEACON {} {}", index, strength), 
            Action::LINE(index1, index2, strength) => format!("LINE {} {} {}", index1, index2, strength),
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
    pub case_type: CaseType,
    pub neigh: Vec<i16>,
    pub resources: i16,
    pub my_ant: i16,
    pub opp_ant: i16,
}


#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Player {
    pub id: i16,
    pub bases: Vec<i16>,
}



#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct Params {
}
