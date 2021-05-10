// Graph module with implementation Graph seaRCh algorithms
use std::vec::Vec;
use std::collections::LinkedList;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use board::*;

use crate::model;

// ReseaRCh shortesd path on the graph between start and target

pub fn resolve_astar< T: Eq + PartialEq + Sized + Hash + Debug >(
    board: &dyn board::Board<T>,
    start: &Rc<dyn board::BoardCase<T>>,
    target: &Rc<dyn board::BoardCase<T>> ) -> LinkedList<Rc<dyn board::BoardCase<T>>> {
    
    let mut targeted_nodes : Vec< model::Way<T> > = Vec::new();
    let mut fisrt_way_ls : LinkedList< Rc<dyn board::BoardCase<T>> > = LinkedList::new();
    let initial_distance = board.distance(start, target);
    fisrt_way_ls.push_back(Rc::clone(&start));
    let first_way : model::Way<T> = model::Way{
        nodes: fisrt_way_ls,
        distance: initial_distance
    };

    targeted_nodes.push(first_way);

    // While target of the best way 
    while targeted_nodes.len() > 0 && targeted_nodes.last().unwrap().nodes.back().unwrap().position() != target.position() {

        let better_way : model::Way<T> = targeted_nodes.pop().unwrap();

        let next_nodes : Vec<Rc<dyn board::BoardCase<T>>> = board.neighbors(better_way.nodes.back().unwrap());


        for next_node in next_nodes {
            let mut new_nodes_path = better_way.nodes.clone();
            let new_distance = board.distance(&next_node, &target);
            new_nodes_path.push_back(Rc::clone(&next_node));
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
    use crate::astar::*;

    #[test]
    fn test_nominal() {


        let board = Hexagon::<i16>::new( 3, &0);

        let best_way = resolve_astar::<i16>( &board, &board.get( &vec!(3, -3, 0) ), &board.get( &vec!(0, 0, 0) ));

        println!("{:?}", best_way);
        assert_eq!(best_way.len(), 4);
        
    }
}
