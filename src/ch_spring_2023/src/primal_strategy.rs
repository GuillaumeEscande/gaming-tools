use crate::model;
use crate::constant;
use crate::helper;
use crate::game_impl;
use game::Strategy;
use board::Board;
use board::BoardCase;
use rand::Rng;
use std::rc::Rc;
use std::cmp::Ordering;


#[derive(Eq,PartialEq,Debug,Clone,Hash)]
pub struct PrimalStrategy {
}

impl Strategy < model::Case, linearhexagon::LinearHexagonCase<model::Case>, model::Player, linearhexagon::LinearHexagon<model::Case>, model::Actions, game_impl::GameImpl> for PrimalStrategy {
    fn compute_action(&self, game: &game_impl::GameImpl) -> model::Actions{
                // Select actions
        let mut actions: Vec<model::Action> = Vec::<model::Action>::new();

        actions.push(model::Action::WAIT());

        return model::Actions{actions : actions}
    }
}
