use crate::model;

use logger;

pub fn print_actions(actions: Vec<model::Action>) {
    logger::output(actions.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(";"));
}