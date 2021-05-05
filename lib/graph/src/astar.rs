// Graph module with implementation Graph seaRCh algorithms
pub mod graph {
    use std::vec::Vec;
    use crate::model::graph;
    use std::collections::LinkedList;
    use std::fmt::Debug;


    // ReseaRCh shortesd path on the graph between start and target

    pub fn resolve_astar< T : graph::Nodeable + Clone + Debug>(
        start: &T,
        target: &T ) -> LinkedList<T> {
        use std::collections::LinkedList;
        
        let mut targeted_nodes : LinkedList< graph::Way<T> > = LinkedList::new();
        let mut fisrt_way_ls : LinkedList<T> = LinkedList::new();
        let initial_distance = start.distance(target);
        fisrt_way_ls.push_back(start.clone());
        let first_way : graph::Way<T> = graph::Way{
            nodes: fisrt_way_ls,
            distance: initial_distance
        };

        targeted_nodes.push_back(first_way);

        // While target of the best way 
        while targeted_nodes.len() > 0 && targeted_nodes.front().unwrap().nodes.back().unwrap() != target {

            let better_way : graph::Way<T> = targeted_nodes.pop_front().unwrap();

            let next_nodes : Vec< T > = better_way.nodes.back().unwrap().nexts();

            for next_node in next_nodes {
                let mut new_nodes_path = better_way.nodes.clone();
                let new_distance = next_node.distance(target);
                new_nodes_path.push_back(next_node);
                let new_way : graph::Way<T> = graph::Way{
                    nodes: new_nodes_path,
                    distance: new_distance
                };
                targeted_nodes.push_back(new_way);
            }

            let mut vec: Vec<_> = targeted_nodes.into_iter().collect();
            vec.sort_unstable_by_key(|k| k.distance);
            targeted_nodes = vec.into_iter().collect();


        }
        if targeted_nodes.len() > 0{
            return targeted_nodes.pop_front().unwrap().nodes;
        }
        else {
            return LinkedList::new();
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::model::graph;
    use crate::astar::graph::*;
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

        impl graph::Nodeable for BoardCase {
            fn nexts(&self) -> Vec< Self >{
                let mut nexts : Vec<BoardCase> = Vec::new();

                if self.x > 1  {
                    nexts.push(BoardCase{
                        x: self.x - 1,
                        y: self.y,
                        x_size: self.x_size,
                        y_size: self.y_size
                    });
                };
                if self.x < (self.x_size - 1) {
                    nexts.push(BoardCase{
                        x: self.x + 1,
                        y: self.y,
                        x_size: self.x_size,
                        y_size: self.y_size
                    });
                };
                if self.y > 1  {
                    nexts.push(BoardCase{
                        x: self.x,
                        y: self.y - 1,
                        x_size: self.x_size,
                        y_size: self.y_size
                    });
                };
                if self.y < (self.y_size - 1) {
                    nexts.push(BoardCase{
                        x: self.x,
                        y: self.y + 1,
                        x_size: self.x_size,
                        y_size: self.y_size
                    });
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

        let best_way : LinkedList<BoardCase> = resolve_astar::<BoardCase>( &origin, &target );

        assert_eq!(best_way.len(), 19);
        
    }
}
