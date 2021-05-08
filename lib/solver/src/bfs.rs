// Graph module with implementation Graph Search algorithms
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

use crate::model::*;

// Research shortest path on the graph between start and target

pub fn breadth_first_search<T: Solvable + Eq + PartialEq + Hash>(
    init_state: Rc<T>,
    max_depth: i32) -> HashMap<Rc<T>, i64> {
    eprintln!("start of bfs, max deep {}", max_depth);

    let mut listed_states: HashMap<Rc<T>, i64> = HashMap::new();
    let mut queue: VecDeque<Rc<T>> = VecDeque::new();
    queue.push_back(init_state);

    let mut depth: i32 = 0;

    while !queue.is_empty() && depth < max_depth {
        let current = queue.pop_back().expect("WTF la queue est vide !");

        for next_state in &current.next_states()
        {
            if !listed_states.contains_key(next_state) {
                queue.push_back(next_state.clone());
                listed_states.insert(next_state.clone(), next_state.value());
            }
        }
        depth += 1;
    }

    return listed_states;
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::LinkedList;
    use std::rc::Rc;

    use crate::model::*;

    use super::*;

    #[derive(Eq, PartialEq, Hash, Clone)]
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
    fn test_bfs() {
        let node2 = Rc::new(DefaultSolvable { value: 9, next_states: LinkedList::<Rc<DefaultSolvable>>::new() });
        let mut next1 = LinkedList::<Rc<DefaultSolvable>>::new();
        next1.push_back(node2.clone());
        let node1 = Rc::new(DefaultSolvable { value: 15, next_states: next1 });

        let result: HashMap<Rc<DefaultSolvable>, i64> = breadth_first_search(node1.clone(), 0);
        assert_eq!(result.len(), 0);

        let result: HashMap::<Rc<DefaultSolvable>, i64> = breadth_first_search(node1.clone(), 1);
        assert_eq!(result.len(), 1);

        let result: HashMap::<Rc<DefaultSolvable>, i64> = breadth_first_search(node1.clone(), 2);
        assert_eq!(result.len(), 1);
    }
}
