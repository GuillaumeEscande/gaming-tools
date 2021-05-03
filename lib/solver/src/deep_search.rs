// Graph module with implementation Graph seaRCh algorithms
pub mod solver {
    use std::collections::HashMap;
    use std::hash::Hash;
    use model::solver::*;
    use crate::model;

    // ReseaRCh shortesd path on the graph between start and target

    pub fn breadth_first_search< T : Solvable + Eq + Hash>(
        initial: T,
        max_deep: i32 ) -> HashMap<T, f64> {
        use std::collections::LinkedList;
        
        let mut listed_states : HashMap<T, f64> = HashMap::new();

        let mut to_test_states : LinkedList<T> = LinkedList::new();

        to_test_states.insert(initial, 0.0);

        let deep_count : i32 = 0;

        while deep_count < max_deep && to_test_states.len() > 0 {

            let next_floor_states : LinkedList<T> = LinkedList::new();

            for node in to_test_states {
                listed_states.insert(node, node.value());
                next_floor_states.append(node.next_states());
            }

        }
        
        return listed_states;
    }

}


#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::hash::Hasher;
    use crate::model::solver::*;
    use super::*;

    #[test]
    fn test_bdf() {

        #[derive(Eq,PartialEq)]
        struct DefaultSolvable {
            value: f64,
            next_states: LinkedList< Self >
        }

        impl Solvable for DefaultSolvable {
            fn value(&self)->f64{
                return self.value;
            }
            fn next_states(&self)->LinkedList< Self >{
                return self.next_states;
            }
        }

        impl Hash for DefaultSolvable {
            fn hash<H>(&self, state: &mut H) where H: Hasher {
                self.canonicalize().hash(state);
            }
        }

        let mut node2 = DefaultSolvable{ value: 9.0, next_states: LinkedList::<DefaultSolvable>::new() };
        let mut node1 = DefaultSolvable{ value: 15.0, next_states: LinkedList::<DefaultSolvable>::new() };


        let result : HashMap::<&DefaultSolvable, f64> = solver::breadth_first_search(
            node1,
            1);

        assert!(result.len() == 0);
    }
}
