mod model;
mod init;
mod choose;

fn main() {
    // Init data
    let id_mapping = model::get_id_mapping();
    let board = init::init_board(&id_mapping);

    // game loop
    loop {
        // Build game
        let mut game = init::init_game(&id_mapping);

        eprintln!("{:?}", game);
        let mut used_sun = 0;
        
        loop {
            if used_sun < game.me.sun {
                break;
            }

            let action = choose::choose1(&mut used_sun, &id_mapping, &board, &mut game);

            action.print();



        }
        model::Action::WAIT("Tu va creuver".to_string()).print();
    }
}
