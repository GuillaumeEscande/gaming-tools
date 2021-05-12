use crate::linear_hexagon;
use board::Board;
use crate::model;
use crate::complete;
use crate::seed;
use crate::grow;

pub fn choose1(
    board: &linear_hexagon::LinearHexagon::<model::Case>,
    game : &mut model::Game,
    params : &model::Params,
    better_cases: &Vec<i16> ) -> () {



    eprintln!("-- CHOOSE PARAMS : ");
    eprintln!("-- - sun restant          = {}", game.me.sun);
    eprintln!("-- - better_cases         = {:?}", better_cases);
    eprintln!("-- - params            = {:?}", params);




    let complete : Option<model::Action> = complete::extract_complete( board, game, params,better_cases );

    let seed_grow : Option<model::Action> = grow::extract_seed_to_grow( board, game, params,better_cases );
    let seed : Option<model::Action> = seed::extract_seed( board, game, params,better_cases );
    let last_grow : Option<model::Action> = grow::extract_grow( board, game, params,better_cases );

   model::Action::WAIT("Je ne sais pas quoi faire".to_string()).print();

}
