// Graph module with implementation Graph Search algorithms
pub mod solver {
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;
    use std::rc::Rc;

    use model::solver::*;

    use crate::model;

// Research shortesd path on the graph between start and target

    pub fn deep_search<T: Solvable + Eq + PartialEq + Hash>(
        initial: Rc<T>) -> HashMap<Rc<T>, i64> {
        let mut listed_states: HashMap<Rc<T>, i64> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_front(initial);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            listed_states.insert(current.clone(), current.value());
            for node in current.next_states() {
                queue.push_front(node);
            }
        }

        return listed_states;
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::LinkedList;
    use std::rc::Rc;

    use crate::model::solver::*;

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
    fn test_deep_search() {
        let node2 = Rc::new(DefaultSolvable { value: 9, next_states: LinkedList::<Rc<DefaultSolvable>>::new() });
        let mut next1 = LinkedList::<Rc<DefaultSolvable>>::new();
        next1.push_back(node2);
        let node1 = Rc::new(DefaultSolvable { value: 15, next_states: next1.clone() });

        let result: HashMap::<Rc<DefaultSolvable>, i64> = solver::deep_search(node1.clone());
        assert_eq!(result.len(), 2);
    }
}
