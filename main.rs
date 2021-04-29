
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
    #[warn(unused_macros)]
    macro_rules! parse_input {
        ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
    }
}
pub mod logger {
    #[warn(dead_code)]
    pub fn debug(message: &str) {
        println!("{}", message);
    }
}
fn main() {
    logger::debug("Hello, world!");
    gamio::output("Hello, world!");
}
