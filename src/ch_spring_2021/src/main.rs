mod model;
mod init;
mod choose;
use std::vec::Vec;

fn main() {
    // Init data
    let id_mapping = model::get_id_mapping();
    let board = init::init_board(&id_mapping);

    let better_cases : Vec<i16> = vec!(0, 1, 2, 3, 4, 5, 6);

    // game loop
    loop {
        // Build game
        let mut game = init::init_game();
        eprintln!("---------- START TURN ----------");

        game.print();
        
        eprintln!("---------- CHOOSE ACTION ----------");

        choose::choose1(&board, &mut game, &better_cases);

    }
}
