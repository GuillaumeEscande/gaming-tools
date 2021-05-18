mod model;
mod init;
mod constant;
mod tools;
mod actions;
mod simu;
use std::rc::Rc;
use solver::breadth_first_search;
use solver::GameSolvable;
use std::marker::PhantomData;

fn main() {
    // Init data
    let board : Rc<linearhexagon::LinearHexagon::<model::Case>> = init::init_board();
    
    let mut game : Rc<model::TreeGame> = Rc::new(model::TreeGame{
        board: board,
        day:-1,
        nutrients:-1,
        me:model::Player{sun:-1, score:-1, is_asleep:false},
        opp:model::Player{sun:-1, score:-1, is_asleep:false},
        trees: vec!(),
    });

    // game loop
    loop {
        // Build game
        init::update_game(&mut game);
        let params = init::init_param(&game); 

        // TODO update board with tree and shadow
        tools::reset_board(&mut game);
        tools::update_tree(&mut game);
        tools::update_shadow(&mut game);
        eprintln!("---------- START TURN ----------");

        game = find_solution(&game);

        eprintln!("---------- CHOOSE ACTION ----------");

    }
}

pub fn find_solution(game : &Rc<model::TreeGame>) -> Rc<model::TreeGame>{
    let gamesolver = solver::GameSolvable::<
        model::Case,
        linearhexagon::LinearHexagonCase::<model::Case>,
        model::Player,
        linearhexagon::LinearHexagon::<model::Case>,
        model::Action,
        model::TreeGame >{
    game : Rc::clone(game),
    _phantom_t: PhantomData,
    _phantom_p: PhantomData,
    _phantom_b: PhantomData,
    _phantom_a: PhantomData,
    _phantom_c: PhantomData };
    
    //solver::breadth_first_search(&Rc::new(gamesolver), 4);
    solver::deep_search(&Rc::new(gamesolver));
    return simu::simulate_turn(&Rc::new(game), &model::Action::WAIT("".to_string()))
}
