mod model;
mod init;
mod constant;
use std::rc::Rc;


fn main() {
    // Init data
    let mut board : Rc<linearhexagon::LinearHexagon::<model::Case>> = init::init_board();

    // game loop
    loop {
        // Build game
        let mut game = init::init_game(&board);
        let params = init::init_param(&game); 
        eprintln!("---------- START TURN ----------");
        
        eprintln!("---------- CHOOSE ACTION ----------");

    }
}
