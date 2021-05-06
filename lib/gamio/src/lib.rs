
pub mod gamio {
    
    #[warn(dead_code)]
    pub fn output(message: &str) {
        println!("{}", message);
    }
    
    #[warn(dead_code)]
    pub fn read_line() -> String {
        use std::io;
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        return input_line;
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn test_output() {
        assert_eq!(3, 3);
    }
}
