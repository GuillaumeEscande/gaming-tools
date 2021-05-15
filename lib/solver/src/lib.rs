use std::collections::LinkedList;
use std::collections::VecDeque;
use std::rc::Rc;
use game::Game;
use board::Board;
use board::BoardCase;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Solvable : Clone + Debug + Eq + PartialEq + Sized {
    fn value(&self)->i64;
    fn next_states(&self)->LinkedList< Rc<Self> >;
}

// Parcours en largeur
pub fn breadth_first_search<T: Solvable + PartialEq>(
    init_state: Rc<T>,
    max_depth: i32) -> LinkedList< ( Rc<T>, i64 ) > {
    eprintln!("start of bfs, max deep {}", max_depth);

    let mut listed_states: LinkedList< ( Rc<T>, i64 ) > = LinkedList::new();
    let mut current_depth: VecDeque::< Rc::<T>> = VecDeque::new();
    current_depth.push_back(init_state);

    let mut depth: i32 = 0;

    while !current_depth.is_empty() && depth < max_depth {
        let mut next_deph : VecDeque::< Rc::< T > > = VecDeque::new();
        for current_state in &current_depth {
            for next_state in &current_state.next_states()
            {
                if listed_states.iter().find(|&x| &x.0 == next_state).is_none() {
                    next_deph.push_back( next_state.clone());
                    listed_states.push_back( ( next_state.clone(), next_state.value() ) );
                }
            }
            depth += 1;
        }
    }

    return listed_states;
}

pub fn deep_search<T: Solvable + PartialEq>(
    initial: Rc<T>) -> LinkedList< ( Rc<T>, i64 ) > {

    let mut listed_states: LinkedList< ( Rc<T>, i64 ) > = LinkedList::new();
    let mut queue = VecDeque::new();
    queue.push_front(initial);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if listed_states.iter().find(|&x| x.0 == current).is_none() {
            listed_states.push_back( (current.clone(), current.value()) );
        }
        for node in current.next_states() {
            queue.push_front(node);
        }
    }

    return listed_states;
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct GameSolvable <T : Eq + PartialEq + Sized + Clone + Debug,
                     C : BoardCase<T>, 
                     P: Eq + PartialEq + Sized + Clone + Debug,
                     B: Board<T, C >,
                     A: Eq + PartialEq + Sized + Clone + Debug,
                     G : Game<T, C, P, B, A> >{
    game : Rc<G>,
    _phantom_t: PhantomData<T>,
    _phantom_p: PhantomData<P>,
    _phantom_b: PhantomData<B>,
    _phantom_a: PhantomData<A>,
    _phantom_c: PhantomData<C>,
}

impl <  T : Eq + PartialEq + Sized + Clone + Debug,
        C : BoardCase<T>, 
        P: Eq + PartialEq + Sized + Clone + Debug,
        B: Board<T, C >,
        A: Eq + PartialEq + Sized + Clone + Debug,
        G : Game<T, C, P, B, A> > Solvable for GameSolvable<T, C, P, B, A, G> {

    fn value(&self)->i64{
        return self.game.score();
    }
    fn next_states(&self)->LinkedList< Rc<Self> >{
        let mut states = LinkedList::< Rc::<Self> >::new();
        for action in self.game.actions(){
            states.push_back(Rc::new(GameSolvable {
                game: self.game.apply(&action),
                _phantom_t: PhantomData,
                _phantom_p: PhantomData,
                _phantom_b: PhantomData,
                _phantom_a: PhantomData,
                _phantom_c: PhantomData,
            }));
        }
        return states;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    use std::rc::Rc;

    use super::*;

    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    struct DefaultSolvable {
        value: i64,
        next_states: LinkedList<Rc<Self>>,
    }

    impl Solvable for DefaultSolvable {
        fn value(&self) -> i64 {
            return self.value;
        }
        fn next_states(&self) -> LinkedList<Rc<Self>> {
            return self.next_states.iter().map(|state| state.clone()).collect();
        }
    }

    #[test]
    fn test_deep_search() {
        let node2 = Rc::new(DefaultSolvable { value: 9, next_states: LinkedList::<Rc<DefaultSolvable>>::new() });
        let mut next1 = LinkedList::<Rc<DefaultSolvable>>::new();
        next1.push_back(node2);
        let node1 = Rc::new(DefaultSolvable { value: 15, next_states: next1.clone() });

        let result: LinkedList< ( Rc<DefaultSolvable>, i64 ) > = deep_search(node1.clone());
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_bfs() {
        let node2 = Rc::new(DefaultSolvable { value: 9, next_states: LinkedList::<Rc<DefaultSolvable>>::new() });
        let mut next1 = LinkedList::<Rc<DefaultSolvable>>::new();
        next1.push_back(node2.clone());
        let node1 = Rc::new(DefaultSolvable { value: 15, next_states: next1 });

        let result: LinkedList< ( Rc<DefaultSolvable>, i64 ) > = breadth_first_search(node1.clone(), 0);
        assert_eq!(result.len(), 0);

        let result: LinkedList< ( Rc<DefaultSolvable>, i64 ) > = breadth_first_search(node1.clone(), 1);
        assert_eq!(result.len(), 1);

        let result: LinkedList< ( Rc<DefaultSolvable>, i64 ) > = breadth_first_search(node1.clone(), 2);
        assert_eq!(result.len(), 1);
    }
}

