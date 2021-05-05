// Graph module with implementation Graph seaRCh algorithms
pub mod graph {
    use std::vec::Vec;
    use crate::model::graph;
    use std::collections::LinkedList;

    // ReseaRCh shortesd path on the graph between start and target

    pub fn resolve_astar< T : graph::Nodeable>(
        start: T,
        target: &T ) -> LinkedList<T> {
        use std::collections::LinkedList;
        
        let mut targeted_nodes : LinkedList< graph::Way<T> > = LinkedList::new();
        let mut fisrt_way_ls : LinkedList<T> = LinkedList::new();
        fisrt_way_ls.push_back(start);
        let first_way = graph::Way(
            nodes: fisrt_way_ls
        );

        targeted_nodes.push_back(first_way);

        // While target of the best way 
        while ! targeted_nodes.front().unwrap().nodes.back().unwrap() == target {
/*
            for (node, cost) in &targeted_nodes {
                for (next_node, next_cost) in node.nexts() {
                    let new_cost = cost + next_cost;
                    if targeted_nodes.contains_key(next_node) {
                        let old_cost = targeted_nodes.get(next_node);
                        if new_cost < old_cost {
                            targeted_nodes.insert(next_node, new_cost);
                        }
                    } else {
                        targeted_nodes.insert(next_node, new_cost);
                    }
                }
            }
            */

        }
        //
        //
        let best_way = Vec::new();
        return best_way;
    }

}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::model;
    use super::*;

    #[test]
    fn test_nominal() {

/*
        let node2 = Rc::<default::DefaultNode>::new(default::DefaultNode{ name: String::from("node2"), nexts: HashMap::new()});
        let link1 = Rc::<default::DefaultLink>::new(default::DefaultLink{ cost: 2.5});
        let node1_next : HashMap< Rc<dyn model::graph::Node>, Rc<dyn model::graph::Link> > = HashMap::new();
        node1_next.insert(node2, link1);
        let node1 = Rc::<default::DefaultNode>::new(default::DefaultNode{ name: String::from("node1"), nexts: node1_next});

        let nodes : Vec< Rc<dyn model::graph::Node> > = vec![node1.clone(), node2.clone()];
        let links : Vec< Rc<dyn model::graph::Link> > = vec![link1.clone()];

        let grah = Rc::<default::DefaultGraph>::new(default::DefaultGraph{
            nodes: nodes,
            links: links});


        let result : Vec::< Rc<dyn model::graph::Node> > = graph::resolve_astar(
            grah,
            node1.clone(),
            node2.clone() );

        assert!(result.len() == 0);
*/
    }
}
