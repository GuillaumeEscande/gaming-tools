mod model;
mod game_impl;
mod init;
mod constant;
mod helper;
mod primal_strategy;

use std::time::Instant;

use board::Board;
use game::Game;
use game::Strategy;



fn main() {
    // Init data
    let number_of_cells = init::init_size();
    let mut cpt_turn = 0;

    // game loop
    loop {
        // Build game
        let mut now = Instant::now();
        let game = init::init_game(size, cpt_turn);
        eprintln!("---------- READ INPUT ----------");
        let board = &game.board;
        game.print();
        eprintln!("Score : {}", game.score());

        eprintln!("Duration : {} ms", now.elapsed().as_millis());
        now = Instant::now();
        eprintln!("---------- START TURN {} ----------", cpt_turn);

        let strategy = primal_strategy::PrimalStrategy{};


        eprintln!("Duration : {} ms", now.elapsed().as_millis());
        eprintln!("------- LIST ALL UNITARY ACTIONS -------");


        
        // Move Robot Defence

        eprintln!("Duration : {} ms", now.elapsed().as_millis());
        eprintln!("-------- CHOOSE ACTIONS --------");
        let action = strategy.compute_action(&game);



        eprintln!("Duration : {} ms", now.elapsed().as_millis());
        eprintln!("-------- PRINT ACTIONS ---------");
        action.print();

        eprintln!("Duration : {} ms", now.elapsed().as_millis());
        eprintln!("---------- END ----------");
        cpt_turn = game.turn;

    }
}
