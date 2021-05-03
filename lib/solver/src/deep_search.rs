// Graph module with implementation Graph Search algorithms
pub mod solver {
    use std::collections::HashMap;
    use std::hash::Hash;
    use model::solver::*;
    use crate::model;

    // Research shortesd path on the graph between start and target

    pub fn deep_search< T : Solvable + Eq + PartialEq + Hash>(
        initial: T ) -> HashMap<T, i64> {
            use std::collections::LinkedList;

            let mut listed_states : HashMap<T, i64> = HashMap::new();

            let mut to_test_states : LinkedList<T> = LinkedList::new();

            to_test_states.push_front(initial);

            while to_test_states.len() > 0 {

                let node = to_test_states.pop_front().unwrap();

                let value: i64 = node.value();
                let next_state = node.next_states();

                listed_states.insert(node, value);
                for node in next_state{
                    to_test_states.push_front(node);
                }

            }

            return listed_states;
    }

}


#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    use std::collections::HashMap;
    use crate::model::solver::*;
    use super::*;

    #[test]
    fn test_deep_search() {

        #[derive(Eq,PartialEq,Hash,Clone)]
        struct DefaultSolvable {
            value: i64,
            next_states: LinkedList< Self >
        }

        impl Solvable for DefaultSolvable {
            fn value(&self)->i64{
                return self.value;
            }
            fn next_states(&self)->LinkedList< Self >{
                return self.next_states.clone();
            }
        }

        let node2 = DefaultSolvable{ value: 9, next_states: LinkedList::<DefaultSolvable>::new() };
        let mut next1 = LinkedList::<DefaultSolvable>::new();
        next1.push_back(node2);
        let node1 = DefaultSolvable{ value: 15, next_states: next1 };


        let result : HashMap::<DefaultSolvable, i64> = solver::deep_search( node1 );
        assert!(result.len() == 2);
    }
}
