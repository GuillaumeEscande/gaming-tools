pub mod graph {
    use std::collections::LinkedList;   

    pub trait Node {
        fn name(self) -> String;
        fn next(self) -> LinkedList< (Box<dyn Link>, Box<dyn Node>) >;
    }
    
    pub trait Link {
        fn cost(self) -> f64;
    }
    
    pub trait Graph {
        fn nodes(self) -> LinkedList< Box<dyn Node> >;
        fn links(self) -> LinkedList< Box<dyn Link> >;
    }
 
    
    pub mod default {
        use std::collections::LinkedList; 
        use crate::model; 

        pub struct DefaultLink {
            pub cost: f64
        }
    
        impl model::graph::Link for DefaultLink {
            fn cost(self) -> f64{
                return self.cost;
            }
        }
    
        pub struct DefaultNode {
            pub name: String,
            pub nexts: LinkedList::< (Box<dyn model::graph::Link>, Box<dyn model::graph::Node>) >
        }
    
        impl model::graph::Node for DefaultNode {
            fn name(self) -> String{
                return self.name;
            }
            fn next(self) -> LinkedList< (Box<dyn model::graph::Link>, Box<dyn model::graph::Node>) >{
                return self.nexts;
            }
        }

        pub struct DefaultGraph {
            pub nodes: LinkedList< Box<dyn model::graph::Node> >,
            pub links: LinkedList< Box<dyn model::graph::Link> >
        }

        impl model::graph::Graph for DefaultGraph {

            fn nodes(self) -> LinkedList< Box<dyn model::graph::Node> >{
                return self.nodes;
            }

            fn links(self) -> LinkedList< Box<dyn model::graph::Link> >{
                return self.links;
            }
        }
    }
}