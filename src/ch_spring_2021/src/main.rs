mod model;
mod init;
mod choose;
mod prepare;
mod complete;
mod grow;
mod seed;
use std::vec::Vec;

fn main() {
    // Init data
    let id_mapping = model::get_id_mapping();
    let board = init::init_board(&id_mapping);

    let better_cases : Vec<i16> = prepare::sort_better_cases(&board);

    // game loop
    loop {
        // Build game
        let mut game = init::init_game(board.size as i16);
        let params = init::init_param(&board, &game); 
        eprintln!("---------- START TURN ----------");

        game.print();
        
        eprintln!("---------- CHOOSE ACTION ----------");

        choose::choose1(&board, &mut game, &params, &better_cases);

    }
}
