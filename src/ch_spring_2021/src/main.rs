mod model;
mod init;

fn main() {
    // Init data
    let id_mapping = model::getIdMapping();
    let board = init::init_board(&id_mapping);

    // game loop
    loop {
        // Build game
        let mut game = init::init_game(&id_mapping);

        eprintln!("{:?}", game);
        let mut used_sun = 0;
        
        loop {
            if used_sun < game.myPlayer.sun {
                break;
            }

            // First action - find COMPLETE
            let complete_index = game.actions.iter().position(|x| matches!(x, model::Action::COMPLETE(_)));
            if complete_index.is_some() {
                let action = game.actions.remove(complete_index.unwrap());
                action.print();
                used_sun += 1;
                continue;
            }

            // Second action - SEED directly on the sun
            let mut seed : Option<usize> = None;
            for i in 0..game.actions.len() {
                match &game.actions[i] {
                    &model::Action::SEED(source, target) => {
                        if target < 6 {
                            seed = Some(i);
                        }
                    },
                    _ => ()
                }
            }
            if seed.is_some() {
                let action = game.actions.remove(seed.unwrap());
                action.print();
                used_sun += 1;
                continue;
            }



        }
        model::Action::WAIT("Tu va creuver".to_string()).print();
    }
}
