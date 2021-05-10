// Graph module with implementation Graph seaRCh algorithms
use std::vec::Vec;
use std::collections::LinkedList;
use std::fmt::Debug;
use std::rc::Rc;
use board::BoardCase;

use crate::model;

// ReseaRCh shortesd path on the graph between start and target

pub fn resolve_astar< B: board::Board, C : board::BoardCase>(
    board: &B,
    start: &C,
    target: &C ) -> LinkedList<Rc<T>> {
    
    let mut targeted_nodes : Vec< model::Way< B > > = Vec::new();
    let mut fisrt_way_ls : LinkedList< Rc< B > > = LinkedList::new();
    let initial_distance = start.distance(target);
    fisrt_way_ls.push_back(Rc::new(start));
    let first_way : model::Way<T> = model::Way{
        nodes: fisrt_way_ls,
        distance: initial_distance
    };

    targeted_nodes.push(first_way);

    // While target of the best way 
    while targeted_nodes.len() > 0 && targeted_nodes.last().unwrap().nodes.back().unwrap() != target {

        let better_way : model::Way<T> = targeted_nodes.pop().unwrap();

        let next_nodes : Vec<Rc<T>> = better_way.nodes.back().unwrap().nexts();


        for next_node in next_nodes {
            let mut new_nodes_path = better_way.nodes.clone();
            let new_distance = next_node.distance(target);
            new_nodes_path.push_back(next_node);
            let new_way : model::Way<T> = model::Way{
                nodes: new_nodes_path,
                distance: new_distance
            };
            targeted_nodes.push(new_way);
        }

        use std::cmp::Reverse;
        targeted_nodes.sort_by_cached_key(|k|  Reverse(k.distance));



    }
    if targeted_nodes.len() > 0{
        return targeted_nodes.pop().unwrap().nodes;
    }
    else {
        return LinkedList::new();
    }
}


#[cfg(test)]
mod tests {
    use crate::model::*;
    use crate::astar::*;
    use std::collections::LinkedList;

    #[test]
    fn test_nominal() {


        let best_way : LinkedList<Rc<BoardCase>> = resolve_astar::<BoardCase>( &Rc::new(origin), &Rc::new(target) );

        assert_eq!(best_way.len(), 19);
        
    }
}

