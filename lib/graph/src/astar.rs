// Graph module with implementation Graph search algorithms
pub mod graph {
    use std::collections::LinkedList;
    use crate::model;

    // Research shortesd path on the graph between start and target

    pub fn resolve_astar(
        graph: Box<dyn model::graph::Graph>,
        start: Box<dyn model::graph::Node>,
        target: Box<dyn model::graph::Node> ) -> LinkedList<Box<dyn model::graph::Node>> {

        let best_way = LinkedList::new();
        return best_way;
    }

}


#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    use crate::model;
    use crate::model::graph::default;
    use super::*;

    #[test]
    fn test_nominal() {

        let mut nodes : LinkedList< Box<dyn model::graph::Node> > = LinkedList::new();
        let mut links : LinkedList< Box<dyn model::graph::Link> > = LinkedList::new();

        let node1 = Box::<default::DefaultNode>::new(default::DefaultNode{
            name: String::from("test"), nexts: LinkedList::new()});
        let link1 = Box::<default::DefaultLink>::new(default::DefaultLink{
            cost: 2.5});
        let node2 = Box::<default::DefaultNode>::new(default::DefaultNode{
            name: String::from("test2"), nexts: LinkedList::new()});

        node1.nexts.push_back((link1, node2));

        nodes.push_back(node1);
        nodes.push_back(node1);

        links.push_back(link1);

        let grah = Box::<default::DefaultGraph>::new(default::DefaultGraph{
            nodes: nodes,
            links: links});


        let result : LinkedList::<Box::<dyn model::graph::Node>> = graph::resolve_astar(
            grah,
            node1,
            node2 );

        assert!(result.len() == 0);
    }
}
