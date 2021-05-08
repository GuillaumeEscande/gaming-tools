use std::io;
mod model;
mod init;
use board::Hexagon;
use board::Board;

fn main() {

    let id_mapping = model::getIdMapping();

    let mut board = init::init_board(&id_mapping);

    // game loop
    loop {

        let game = init::init_game(&id_mapping, &mut board);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        println!("WAIT");
    }
}
