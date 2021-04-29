pub mod graph {
    trait Node {
        fn name(&self)->str;
    }
    
    trait Link {
        fn quack(&self);
    }
    
    trait Graph {
        fn quack(&self);
    }
 
    
    
    #[cfg(test)]
    mod tests {
    
        #[test]
        fn test_output() {
            assert_eq!(3, 3);
        }
    }
}