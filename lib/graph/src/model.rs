pub mod graph {
    use std::collections::HashMap;
    use std::rc::Rc;

    pub trait Node {
        fn get_name(self) -> String;
        fn nexts(self) -> HashMap< Rc<dyn Node>, Rc<dyn Link> >;
    }
    
    pub trait Link {
        fn cost(self) -> f64;
    }
    
    pub trait Graph {
        fn nodes(self) -> Vec< Rc<dyn Node> >;
        fn links(self) -> Vec< Rc<dyn Link> >;
    }
 
    
    pub mod default {
        use std::collections::HashMap; 
        use std::rc::Rc;
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
            pub nexts: HashMap< Rc<dyn model::graph::Node>, Rc<dyn model::graph::Link> >
        }
    
        impl model::graph::Node for DefaultNode {
            fn get_name(self) -> String{
                return self.name;
            }
            fn nexts(self) -> HashMap< Rc<dyn model::graph::Node>, Rc<dyn model::graph::Link> > {
                return self.nexts;
            }
        }

        impl std::hash::Hash for DefaultNode {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.name.hash(state);
            }
        }

        impl PartialEq for DefaultNode {
            fn eq(&self, other: &DefaultNode) -> bool {
                self.name == other.name
            }
        }


        pub struct DefaultGraph {
            pub nodes: Vec< Rc<dyn model::graph::Node> >,
            pub links: Vec< Rc<dyn model::graph::Link> >
        }

        impl model::graph::Graph for DefaultGraph {

            fn nodes(self) -> Vec< Rc<dyn model::graph::Node> >{
                return self.nodes;
            }

            fn links(self) -> Vec< Rc<dyn model::graph::Link> >{
                return self.links;
            }
        }
    }
}