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
        eprintln!("---------- START TURN ----------");

        game.print();

        let mut used_sun = 0;
        
        eprintln!("Sun points : {}", game.me.sun);
        if game.me.sun <= used_sun {
            eprintln!("---------- WAIT ----------");
            model::Action::WAIT("Tu va creuver".to_string()).print();
        } else {
            eprintln!("---------- CHOOSE ACTION ----------");

            let action = choose::choose1(&mut used_sun, &id_mapping, &board, &mut game);

            action.print();
        }

    }
}
