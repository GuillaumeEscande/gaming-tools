// Graph module with implementation Graph seaRCh algorithms
pub mod graph {
    use std::vec::Vec;
    use std::rc::Rc;
    use crate::model;

    // ReseaRCh shortesd path on the graph between start and target

    pub fn resolve_astar(
        graph: Rc<dyn model::graph::Graph>,
        start: Rc<dyn model::graph::Node>,
        target: Rc<dyn model::graph::Node> ) -> Vec<Rc<dyn model::graph::Node>> {
        use std::collections::HashMap;
        
        let mut targeted_nodes : HashMap<Rc<dyn model::graph::Node>, f64> = HashMap::new();

        targeted_nodes.insert(start, 0.0);

        while ! targeted_nodes.contains_key(target) {

            for (node, cost) in &targeted_nodes {
                for (next_node, link) in node.nexts() {
                    let new_cost = cost + link.cost();
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
    use crate::model::graph::default;
    use super::*;

    #[test]
    fn test_nominal() {


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
    }
}
