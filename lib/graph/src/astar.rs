// Graph module with implementation Graph seaRCh algorithms
use std::vec::Vec;
use std::collections::LinkedList;
use std::fmt::Debug;
use std::rc::Rc;

use crate::model;

// ReseaRCh shortesd path on the graph between start and target

pub fn resolve_astar< T : model::Nodeable + Clone + Debug>(
    start: &Rc<T>,
    target: &Rc<T> ) -> LinkedList<Rc<T>> {
    
    let mut targeted_nodes : Vec< model::Way<T> > = Vec::new();
    let mut fisrt_way_ls : LinkedList<Rc<T>> = LinkedList::new();
    let initial_distance = start.distance(target);
    fisrt_way_ls.push_back(start.clone());
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

        #[derive(Hash, Clone, Eq, PartialEq, Debug)]
        pub struct BoardCase{
            pub x : i64,
            pub y : i64,
            pub x_size : i64,
            pub y_size : i64,
        }

        impl model::Nodeable for BoardCase {
            fn nexts(&self) -> Vec< Rc< Self > >{
                let mut nexts : Vec<Rc<BoardCase>> = Vec::new();

                if self.x > 1  {
                    nexts.push(Rc::new(BoardCase{
                        x: self.x - 1,
                        y: self.y,
                        x_size: self.x_size,
                        y_size: self.y_size
                    }));
                };
                if self.x < (self.x_size - 1) {
                    nexts.push(Rc::new(BoardCase{
                        x: self.x + 1,
                        y: self.y,
                        x_size: self.x_size,
                        y_size: self.y_size
                    }));
                };
                if self.y > 1  {
                    nexts.push(Rc::new(BoardCase{
                        x: self.x,
                        y: self.y - 1,
                        x_size: self.x_size,
                        y_size: self.y_size
                    }));
                };
                if self.y < (self.y_size - 1) {
                    nexts.push(Rc::new(BoardCase{
                        x: self.x,
                        y: self.y + 1,
                        x_size: self.x_size,
                        y_size: self.y_size
                    }));
                };
                return nexts;
            }
            fn distance(&self, target: &BoardCase) -> i64{
                let a_x = self.x as f64;
                let a_y = self.y as f64;
                let b_x = target.x as f64;
                let b_y = target.y as f64;
                let distance = ( ( (b_x - a_x).powi(2) + (b_y - a_y).powi(2) ).sqrt() * 10.0 ) as i64;

                return distance;
            }
        }

        let origin: BoardCase = BoardCase{
            x: 0,
            y: 0,
            x_size: 10,
            y_size: 10,
        };

        let target: BoardCase = BoardCase{
            x: 9,
            y: 9,
            x_size: 10,
            y_size: 10,
        };

        let best_way : LinkedList<Rc<BoardCase>> = resolve_astar::<BoardCase>( &Rc::new(origin), &Rc::new(target) );

        assert_eq!(best_way.len(), 19);
        
    }
}

