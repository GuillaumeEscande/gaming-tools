// Graph module with implementation Graph Search algorithms
pub mod solver {
    use std::collections::HashMap;
    use std::hash::Hash;
    use model::solver::*;
    use crate::model;

    // Research shortesd path on the graph between start and target

    pub fn breadth_first_search< T : Solvable + Eq + PartialEq + Hash>(
        initial: T,
        max_deep: i32 ) -> HashMap<T, i64> {
        use std::collections::LinkedList;

        let mut listed_states : HashMap<T, i64> = HashMap::new();

        let mut to_test_states : LinkedList<T> = LinkedList::new();

        to_test_states.push_back(initial);

        let deep_count : i32 = 0;

        while deep_count < max_deep && to_test_states.len() > 0 {

            let mut next_floor_states : LinkedList<T> = LinkedList::new();

            for node in to_test_states {
                let value: i64 = node.value();
                let mut next_state = node.next_states();

                listed_states.insert(node, value);
                next_floor_states.append(&mut next_state);
            }

            to_test_states = next_floor_states;
            deep_count++;

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
    fn test_bdf() {

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


        let result : HashMap::<DefaultSolvable, i64> = solver::breadth_first_search( node1, 0 );
        assert!(result.len() == 0);
        *
        let result : HashMap::<DefaultSolvable, i64> = solver::breadth_first_search( node1, 1 );
        assert!(result.len() == 1);

        let result : HashMap::<DefaultSolvable, i64> = solver::breadth_first_search( node1, 2 );
        assert!(result.len() == 1);
    }
}
