mod model;
mod init;
mod constant;
use std::rc::Rc;


fn main() {
    // Init data
    let board : Rc<linearhexagon::LinearHexagon::<model::Case>> = init::init_board();

    // game loop
    loop {
        // Build game
        let mut game = init::init_game(board.size as i16, board);
        let params = init::init_param(&board, &game); 
        eprintln!("---------- START TURN ----------");
        
        eprintln!("---------- CHOOSE ACTION ----------");


    }
}
